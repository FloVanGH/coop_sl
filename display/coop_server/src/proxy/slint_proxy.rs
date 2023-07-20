// SPDX-FileCopyrightText: 2022 Florian Blasius <co_sl@tutanota.com>
// SPDX-License-Identifier: MIT

use super::ServerProxy;

use slint::private_unstable_api::re_exports::{KeyEvent, PointerEvent};
use slint::SharedString;

/// `SlintProxy` is a wrapper for `ServerProxy` and make it easier to send events from a `Slint` application to the server.
#[derive(Clone, Debug)]
pub struct SlintProxy {
    proxy: ServerProxy,
}

impl SlintProxy {
    /// Opens a new client application on the given path.
    pub fn open_client(&self, path: impl Into<String>) {
        self.proxy.open_client(path);
    }

    /// Sends a pointer event to the client..
    pub fn send_pointer_event(&self, key: SharedString, x: f32, y: f32, event: PointerEvent) {
        let button = match event.button {
            slint::platform::PointerEventButton::Left => coop_protocol::PointerEventButton::Left,
            slint::platform::PointerEventButton::Right => coop_protocol::PointerEventButton::Right,
            slint::platform::PointerEventButton::Middle => {
                coop_protocol::PointerEventButton::Middle
            }
            _ => coop_protocol::PointerEventButton::None,
        };

        match event.kind {
            slint::private_unstable_api::re_exports::PointerEventKind::Cancel => {
                self.proxy.send_pointer_canceled_event(key);
            }
            slint::private_unstable_api::re_exports::PointerEventKind::Down => {
                self.proxy.send_pointer_pressed_event(key, x, y, button);
            }
            slint::private_unstable_api::re_exports::PointerEventKind::Up => {
                self.proxy
                    .send_pointer_released_event(key.clone(), x, y, button);
                self.proxy.send_pointer_exit_event(key);
            }
        };
    }

    /// Sends a key pressed event to the client of the given key.
    pub fn send_key_pressed_event(&self, key: SharedString, event: KeyEvent) {
        self.proxy.send_key_pressed_event(key, event.text);
    }

    /// Sends a key released event to the client of the given key.
    pub fn send_key_released_event(&self, key: SharedString, event: KeyEvent) {
        self.proxy.send_key_pressed_event(key, event.text);
    }

    /// Sends a resize event.‚
    // pub fn send_resize_event(&self, key: SharedString, width: f32, height: f32) {
    //     self.proxy.send_resize_event(key, width, height);
    // }

    /// Closes the client with the given key.
    pub fn close_client(&self, key: impl Into<String>) {
        self.proxy.close_client(key);
    }

    /// Closes all clients and the server.
    pub fn close(&self) {
        self.proxy.close();
    }
}

impl From<ServerProxy> for SlintProxy {
    fn from(proxy: ServerProxy) -> Self {
        Self { proxy }
    }
}
