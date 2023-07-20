// SPDX-FileCopyrightText: 2022 Florian Blasius <co_sl@tutanota.com>
// SPDX-License-Identifier: MIT

use coop_protocol::{ClientEvent, PointerEventButton};
use slint::platform;
use slint::LogicalPosition;

/// Represents `WindowEvent`, Resize or Quit.
#[derive(Debug, PartialEq)]
pub enum Event {
    /// Slint `WindowEvent`.
    WindowEvent(platform::WindowEvent),

    /// Resize event.
    Resize(slint::LogicalSize),

    /// Window quit event.
    Quit,
}

impl From<slint::platform::WindowEvent> for Event {
    fn from(event: platform::WindowEvent) -> Self {
        Event::WindowEvent(event)
    }
}

/// Reads all events from the given `Window` and converts them into a vec of `Event`.
pub fn read(coop_window: &coop_client::Window) -> Vec<Event> {
    coop_window.events().iter().map(|e| convert(e)).collect()
}

pub fn convert(in_event: &coop_protocol::ClientEvent) -> Event {
    match in_event {
        ClientEvent::Close => Event::Quit,
        ClientEvent::PointerPressed { position, button } => platform::WindowEvent::PointerPressed {
            position: LogicalPosition::new(position.0, position.1),
            button: convert_button(button),
        }
        .into(),
        ClientEvent::PointerRelease { position, button } => {
            platform::WindowEvent::PointerReleased {
                position: LogicalPosition::new(position.0, position.1),
                button: convert_button(button),
            }
            .into()
        }
        // todo: what to do with pointer canceled?
        ClientEvent::PointerCanceled => todo!(),
        ClientEvent::PointerExited => platform::WindowEvent::PointerExited.into(),
        // FIXME: use SharedString after it is updated by Slint
        ClientEvent::KeyPressed { text } => platform::WindowEvent::KeyPressed {
            text: text.chars().into_iter().next().unwrap().into(),
        }
        .into(),
        ClientEvent::KeyReleased { text } => platform::WindowEvent::KeyReleased {
            text: text.chars().into_iter().next().unwrap().into(),
        }
        .into(),
    }
}

fn convert_button(button: &coop_protocol::PointerEventButton) -> platform::PointerEventButton {
    match button {
        PointerEventButton::None => slint::platform::PointerEventButton::Other,
        PointerEventButton::Left => slint::platform::PointerEventButton::Left,
        PointerEventButton::Right => slint::platform::PointerEventButton::Right,
        PointerEventButton::Middle => slint::platform::PointerEventButton::Middle,
    }
}

// FIXME tests for convert convertbutton and read
