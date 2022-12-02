// SPDX-FileCopyrightText: 2022 Florian Blasius <co_sl@tutanota.com>
// SPDX-License-Identifier: MIT

use coop_protocol::*;

use std::env;
use tokio_unix_ipc::{Receiver, Sender};

/// Use this to generate a window with the co window manager and interact with it.
pub struct Window {
    width: f32,
    height: f32,
    _title: String,
    sender: Sender<ClientMessage>,
    receiver: Receiver<ClientMessage>,
    key: String,
    is_async: bool,
    runtime: tokio::runtime::Runtime,
}

impl Default for Window {
    fn default() -> Self {
        Self::new(0.0, 0.0, String::default(), false)
    }
}

impl Window {
    /// Creates a new window with default settings.
    pub fn new(width: f32, height: f32, title: String, is_async: bool) -> Self {
        let runtime = tokio::runtime::Runtime::new().unwrap();
        let creation_result = runtime.block_on(async {
            let server_key = env::var(coop_protocol::SERVER_KEY).expect("Cannot read server key");
            let client_key = env::var(coop_protocol::CLIENT_KEY).expect("Cannot read client key");
            let client_path =
                env::var(coop_protocol::CLIENT_PATH).expect("Cannot read client path");

            let receiver = Receiver::<ClientMessage>::connect(server_key.clone())
                .await
                .unwrap();
            let message = receiver.recv().await.unwrap();

            if let ClientMessage::Create(sender) = message {
                sender
                    .send(ClientMessage::Response(ClientResponse {
                        key: client_key.clone(),
                        message: ClientResponseMessage::Created(CreationResponse {
                            title: title.clone(),
                            path: client_path,
                            width,
                            height,
                        }),
                    }))
                    .await
                    .expect("Cannot send creation message to backend.");

                return Some((sender, receiver, client_key));
            }
            None
        });

        let (sender, receiver, key) =
            creation_result.expect("Cannot establish connection to backend.");

        Self {
            width,
            height,
            _title: title,
            sender,
            receiver,
            key,
            is_async,
            runtime,
        }
    }

    /// Draws the frame buffer to the window.
    pub fn draw(&self, buffer: &[u8], width: f32, height: f32) {
        self.send(ClientMessage::Response(ClientResponse {
            key: self.key.clone(),
            message: ClientResponseMessage::Draw(DrawResponse {
                buffer: buffer.to_vec(),
                width,
                height,
            }),
        }));
    }

    /// Returns the current width of the client window.
    pub fn width(&self) -> f32 {
        self.width
    }

    /// Returns the current height of the client window.
    pub fn height(&self) -> f32 {
        self.height
    }

    /// Returns the current size of the client window.
    pub fn size(&self) -> (f32, f32) {
        (self.width, self.height)
    }

    /// Request list of events.
    pub fn events(&self) -> Vec<ClientEvent> {
        let mut events = vec![];

        'blocking: loop {
            let event = self.runtime.block_on(async {
                if let Ok(message) = self.receiver.recv().await {
                    match message {
                        ClientMessage::Create(_) => unreachable!("Client is already created."),
                        ClientMessage::Response(_) => {
                            unreachable!("Only the client send client responses")
                        }
                        ClientMessage::Event(e) => return Some(e),
                        ClientMessage::Closed(_) => {
                            unreachable!("Only the client send closed responses")
                        }
                    }
                }

                None
            });

            if let Some(event) = event {
                events.push(event);
            }

            if !self.is_async && events.is_empty() {
                std::thread::yield_now();
            } else {
                break 'blocking;
            }
        }

        events
    }

    /// Clean close the client window.
    pub fn close(&self) {
        self.send(ClientMessage::Closed(self.key.clone()));
    }

    fn send(&self, message: ClientMessage) {
        self.runtime.block_on(async {
            self.sender
                .send(message)
                .await
                .expect("Cannot send message to server.");
        });
    }
}
