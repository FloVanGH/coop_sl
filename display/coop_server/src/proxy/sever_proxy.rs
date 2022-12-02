// SPDX-FileCopyrightText: 2022 Florian Blasius <co_sl@tutanota.com>
// SPDX-License-Identifier: MIT

use coop_protocol::{ClientEvent, PointerEventButton};

use tokio::sync::mpsc::UnboundedSender;

/// Used to send messages from `ServerProxy` to  `Server`.
#[derive(Clone, Debug)]
pub enum ServerProxyMessage {
    /// A event that is send to a client.
    SendEvent { key: String, event: ClientEvent },

    /// Opens a client.
    OpenClient(String),

    /// Closes a client.
    CloseClient(String),

    /// Closes all clients and the server.
    Close,
}

/// `ServerProxy` is used to communicate with the `coop_server::Server`.
#[derive(Clone, Debug)]
pub struct ServerProxy {
    sender: UnboundedSender<ServerProxyMessage>,
}

impl ServerProxy {
    /// Creates a new `ServerProxy`.
    pub fn new(sender: UnboundedSender<ServerProxyMessage>) -> Self {
        Self { sender }
    }

    /// Opens a new client application on the given path.
    pub fn open_client(&self, path: impl Into<String>) {
        self.sender
            .send(ServerProxyMessage::OpenClient(path.into()))
            .expect("Cannot send open client message to server.");
    }

    /// Sends an event to the given client.
    pub fn send_client_event(&self, key: impl Into<String>, event: ClientEvent) {
        self.sender
            .send(ServerProxyMessage::SendEvent {
                key: key.into(),
                event,
            })
            .expect("Cannot send event message to server.");
    }

    /// Sends a pointer pressed event to the client of the given key.
    pub fn send_pointer_pressed_event(
        &self,
        key: impl Into<String>,
        x: f32,
        y: f32,
        button: PointerEventButton,
    ) {
        self.send_client_event(
            key,
            ClientEvent::PointerPressed {
                position: (x, y),
                button,
            },
        );
    }

    /// Sends a pointer released event to the client of the given key.
    pub fn send_pointer_released_event(
        &self,
        key: impl Into<String>,
        x: f32,
        y: f32,
        button: PointerEventButton,
    ) {
        self.send_client_event(
            key,
            ClientEvent::PointerRelease {
                position: (x, y),
                button,
            },
        );
    }

    /// Sends a pointer canceled event to the client of the given key.
    pub fn send_pointer_canceled_event(&self, key: impl Into<String>) {
        self.send_client_event(key, ClientEvent::PointerCanceled);
    }

    /// Sends a pointer exit event to the client of the given key.
    pub fn send_pointer_exit_event(&self, key: impl Into<String>) {
        self.send_client_event(key, ClientEvent::PointerExited);
    }

    /// Sends a key pressed event to the client of the given key.
    pub fn send_key_pressed_event(&self, key: impl Into<String>, text: impl Into<String>) {
        self.send_client_event(key, ClientEvent::KeyPressed { text: text.into() });
    }

    /// Sends a key released event to the client of the given key.
    pub fn send_key_released_event(&self, key: impl Into<String>, text: impl Into<String>) {
        self.send_client_event(key, ClientEvent::KeyReleased { text: text.into() });
    }

    /// Closes the client with the given key.
    pub fn close_client(&self, key: impl Into<String>) {
        self.sender
            .send(ServerProxyMessage::CloseClient(key.into()))
            .expect("Cannot send close client message to server.");
    }

    /// Closes all clients and the server.
    pub fn close(&self) {
        self.sender
            .send(ServerProxyMessage::Close)
            .expect("Cannot send close message to server.");
    }
}
