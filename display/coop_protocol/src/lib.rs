// SPDX-FileCopyrightText: 2022 Florian Blasius <co_sl@tutanota.com>
// SPDX-License-Identifier: MIT

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// Env key to identify the server.
pub const SERVER_KEY: &str = "SERVER_KEY";

/// Env key to identify a client.
pub const CLIENT_KEY: &str = "CLIENT_KEY";

/// Origin client path.
pub const CLIENT_PATH: &str = "CLIENT_PATH";

/// Response from the client with the frame buffer.
#[derive(Deserialize, Serialize, Debug, Clone, JsonSchema)]
pub struct DrawResponse {
    /// Frame buffer to draw.
    pub buffer: Vec<u8>,

    /// Current width to draw.
    pub width: f32,

    /// Current height to draw.
    pub height: f32,
}

/// Response from the client that the client is created.
#[derive(Deserialize, Serialize, Debug, Clone, JsonSchema)]
pub struct CreationResponse {
    /// Title of the client.
    pub title: String,

    /// Origin path.
    pub path: String,

    /// Initial frame buffer width of the client.
    pub width: f32,

    /// Initial frame buffer height of the client.
    pub height: f32,
}

/// Response message send from the client.
#[derive(Deserialize, Serialize, Debug, Clone, JsonSchema)]
pub enum ClientResponseMessage {
    /// Client is created.
    Created(CreationResponse),

    /// Client buffer to draw.
    Draw(DrawResponse),

    /// The client is closed.
    Closed,

    /// Message from server if all clients are closed
    AllClosed,
}

/// Response send from the client.
#[derive(Deserialize, Serialize, Debug, Clone, JsonSchema)]
pub struct ClientResponse {
    /// Connection key of the client.
    pub key: String,

    /// Response message.
    pub message: ClientResponseMessage,
}

/// Defines a button of a pointer event.
#[derive(Deserialize, Serialize, Debug, Clone, JsonSchema)]
pub enum PointerEventButton {
    None,
    Left,
    Right,
    Middle,
}

// Event that is send to the client.
#[derive(Deserialize, Serialize, Clone, Debug)]
pub enum ClientEvent {
    /// The client should be closed.
    Close,

    /// Pointer is pressed.
    PointerPressed {
        /// Current position of the button.
        position: (f32, f32),

        /// Pointer button type.
        button: PointerEventButton,
    },

    /// Pointer is released.
    PointerRelease {
        /// Current position of the button.
        position: (f32, f32),

        /// Pointer button type.
        button: PointerEventButton,
    },

    /// The pointer exited the window.
    PointerExited,

    /// Pointer event is canceled.
    PointerCanceled,
    /// A key was pressed.
    KeyPressed {
        /// The unicode representation of the key pressed.
        text: String,
    },

    /// A key was pressed.
    KeyReleased {
        /// The unicode representation of the key released.
        text: String,
    },
}

/// Messages between client and server.
#[derive(Deserialize, Serialize, Debug)]
pub enum ClientMessage {
    /// Request to open an create a client.
    Create(tokio_unix_ipc::Sender<ClientMessage>),

    /// Send an event to the client.
    Event(ClientEvent),

    /// Responses from client to server.
    Response(ClientResponse),

    /// Response that the client is closed
    Closed(String),
}
