// SPDX-FileCopyrightText: 2022 Florian Blasius <co_sl@tutanota.com>
// SPDX-License-Identifier: MIT

use coop_protocol::*;
use std::collections::HashMap;
use std::thread;

use tokio::sync::mpsc;

pub mod proxy;
pub mod result;

use result::*;

/// `Server` is used to work with `co_clients` and handle the communication between window manager and the clients.
pub struct Server {
    proxy: proxy::ServerProxy,

    client_response_receiver: mpsc::UnboundedReceiver<ClientResponse>,
    server_loop: thread::JoinHandle<()>,
}

impl Server {
    /// Creates a new server with default settings.
    pub fn new() -> Self {
        let (proxy_sender, proxy_receiver) = mpsc::unbounded_channel();
        let (client_response_sender, client_response_receiver) = mpsc::unbounded_channel();

        let server_loop = thread::spawn(move || {
            tokio::runtime::Runtime::new()
                .unwrap()
                .block_on(server_loop(proxy_receiver, client_response_sender))
                .expect("Crash on server loop.")
        });

        Self {
            proxy: proxy::ServerProxy::new(proxy_sender),
            client_response_receiver,
            server_loop,
        }
    }

    /// Gets a proxy to communicate with the server.
    pub fn proxy(&self) -> proxy::ServerProxy {
        self.proxy.clone()
    }

    #[cfg(feature = "slint")]
    /// Gets a proxy to communicate with the server easily from a `Slint` application.
    pub fn slint_proxy(&self) -> proxy::SlintProxy {
        self.proxy().into()
    }

    /// Reads the next client response
    pub fn client_response(&mut self) -> Option<ClientResponse> {
        self.client_response_receiver.blocking_recv()
    }

    pub fn join(self) -> thread::Result<()> {
        self.proxy.close();
        self.server_loop.join()
    }
}

async fn server_loop(
    mut proxy_receiver: mpsc::UnboundedReceiver<proxy::ServerProxyMessage>,
    client_response_sender: mpsc::UnboundedSender<ClientResponse>,
) -> ServerResult<()> {
    let mut client_proxy_pool = proxy::ClientProxyPool::new();
    let mut client_loops: HashMap<String, thread::JoinHandle<()>> = HashMap::new();

    'server_loop: loop {
        if let Some(message) = proxy_receiver.recv().await {
            match message {
                proxy::ServerProxyMessage::SendEvent { key, event } => {
                    client_proxy_pool.send_event(&key, event).await
                }
                proxy::ServerProxyMessage::OpenClient(path) => {
                    let (key, client_receiver) = client_proxy_pool.open(path).await?;

                    let client_loop = thread::spawn({
                        let client_response_sender = client_response_sender.clone();
                        move || {
                            tokio::runtime::Runtime::new()
                                .unwrap()
                                .block_on(client_loop(client_response_sender, client_receiver));
                        }
                    });

                    client_loops.insert(key, client_loop);
                }
                proxy::ServerProxyMessage::CloseClient(key) => {
                    client_proxy_pool.request_close(&key).await;
                    client_loops
                        .remove(&key)
                        .expect("Cannot reach client loop")
                        .join()
                        .expect("Cannot join client loop.");
                    client_proxy_pool.kill_client(&key);
                }
                proxy::ServerProxyMessage::Close => {
                    client_proxy_pool.request_close_all().await;
                    break 'server_loop;
                }
            }
        }
    }

    for (_, client_loop) in client_loops {
        client_loop.join().expect("Cannot join client loop close");
    }

    client_proxy_pool.kill_all();

    client_response_sender
        .send(ClientResponse {
            key: String::default(),
            message: ClientResponseMessage::AllClosed,
        })
        .expect("Cannot send all closed message");

    Ok(())
}

async fn client_loop(
    client_response_sender: mpsc::UnboundedSender<ClientResponse>,
    client_receiver: tokio_unix_ipc::Receiver<ClientMessage>,
) {
    'client_loop: loop {
        if let Ok(message) = client_receiver.recv().await {
            match message {
                ClientMessage::Create(_) => unreachable!("Client cannot send create messages"),
                ClientMessage::Event(_) => unreachable!("Client cannot send events"),
                ClientMessage::Response(response) => client_response_sender
                    .send(response)
                    .expect("Cannot send inner message."),
                ClientMessage::Closed(key) => {
                    client_response_sender
                        .send(ClientResponse {
                            key,
                            message: ClientResponseMessage::Closed,
                        })
                        .expect("Cannot send inner message.");

                    break 'client_loop;
                }
            }
        }
    }
}
