// SPDX-FileCopyrightText: 2022 Florian Blasius <co_sl@tutanota.com>
// SPDX-License-Identifier: MIT

use std::path::*;
use std::process::Command;
use std::{collections::HashMap, process::Child};

use coop_protocol::{ClientEvent, ClientMessage};
use tokio_unix_ipc::Bootstrapper;

use crate::result::*;

/// Used to send messages from `Server` to  client.
#[derive(Debug)]
pub struct ClientProxy {
    sender: Bootstrapper,
    client: Child,
}

impl ClientProxy {
    /// Creates a new client proxy.
    pub fn new(client: Child, sender: Bootstrapper) -> Self {
        Self { client, sender }
    }

    /// Sends an event to the client.
    pub async fn send_event(&self, event: ClientEvent) {
        self.sender
            .send(ClientMessage::Event(event))
            .await
            .expect("Cannot send event to client");
    }

    /// Request to close the client.
    pub async fn request_close(&mut self) {
        self.send_event(ClientEvent::Close).await;
    }

    /// Ends the client.
    pub fn kill(&mut self) {
        self.client.kill().ok();
        self.client.wait().ok();
    }
}

/// Manages a pool of client proxies.
#[derive(Debug)]
pub struct ClientProxyPool {
    proxies: HashMap<String, ClientProxy>,
    connection_count: i32,
}

impl ClientProxyPool {
    /// Create a new pool.
    pub fn new() -> Self {
        Self {
            proxies: HashMap::new(),
            connection_count: 0,
        }
    }

    /// Opens a new client and returns a client message receiver.
    pub async fn open<P>(
        &mut self,
        path: P,
    ) -> ServerResult<(String, tokio_unix_ipc::Receiver<ClientMessage>)>
    where
        P: AsRef<Path>,
    {
        let sender = Bootstrapper::new().unwrap();
        let client_path = path
            .as_ref()
            .to_path_buf()
            .into_os_string()
            .into_string()
            .map_err(|e| Err(ServerError::CannotOpenClient(format!("{:?}", e))))?;

        let connection_key = format!("{client_path}_{}", self.connection_count);
        self.connection_count += 1;

        let client = Command::new(path.as_ref())
            .env(coop_protocol::SERVER_KEY, sender.path())
            .env(coop_protocol::CLIENT_KEY, connection_key.clone())
            .env(coop_protocol::CLIENT_PATH, client_path)
            .spawn()
            .map_err(|e| {
                Err(ServerError::CannotOpenClient(format!(
                    "Cannot open client {:?}",
                    e
                )))
            })?;

        let (client_sender, client_receiver) = tokio_unix_ipc::channel().unwrap();

        sender
            .send(ClientMessage::Create(client_sender))
            .await
            .map_err(|_| {
                ServerError::CannotOpenClient("Cannot establish connection with client".into())
            })?;

        self.proxies
            .insert(connection_key.clone(), ClientProxy::new(client, sender));

        Ok((connection_key, client_receiver))
    }

    /// Sends an event to the given client.
    pub async fn send_event(&self, key: &String, event: ClientEvent) {
        if let Some(proxy) = self.proxies.get(key) {
            proxy.send_event(event).await;
        }
    }

    /// Closes a client.
    pub async fn request_close(&mut self, key: &String) {
        if let Some(proxy) = self.proxies.get_mut(key) {
            proxy.request_close().await;
        }
    }

    /// End the given client
    pub fn kill_client(&mut self, key: &String) {
        if let Some(proxy) = self.proxies.get_mut(key) {
            proxy.kill();
        }

        self.proxies.remove(key);
    }

    /// Ends all clients.
    pub fn kill_all(&mut self) {
        for (_, proxy) in &mut self.proxies {
            proxy.kill();
        }

        self.proxies.clear();
    }

    /// Request to close all clients.
    pub async fn request_close_all(&mut self) {
        for (_, proxy) in &mut self.proxies {
            proxy.request_close().await;
        }
    }
}
