// SPDX-FileCopyrightText: 2022 Florian Blasius <co_sl@tutanota.com>
// SPDX-License-Identifier: MIT

use std::cell::Cell;

/// Represents `WindowEvent`, Resize or Quit.
#[derive(Debug, PartialEq)]
pub enum Event {
    /// Slint `WindowEvent`.
    WindowEvent(slint::platform::WindowEvent),

    /// Resize event.
    Resize(slint::LogicalSize),

    /// Window quit event.
    Quit,
}

impl From<slint::platform::WindowEvent> for Event {
    fn from(event: slint::platform::WindowEvent) -> Self {
        Event::WindowEvent(event)
    }
}

#[derive(Debug, Default, Copy, Clone, PartialEq)]
struct ButtonStates {
    left: bool,
    middle: bool,
    right: bool,
}

impl ButtonStates {
    fn new(left: bool, middle: bool, right: bool) -> Self {
        Self {
            left,
            middle,
            right,
        }
    }
}

/// Reads in events from a orbclient Window and converts them into Slint `WindowEvents`s.
#[derive(Debug, Default, Clone, PartialEq)]
pub struct EventReader {
    button_states: Cell<ButtonStates>,
    pointer_position: Cell<slint::LogicalPosition>,
}

impl EventReader {
    /// Reads all events from the given `Window` and converts them into a vec of `Event`.
    pub fn read(&self, orb_window: &mut orbclient::Window) -> Vec<Event> {
        orb_window.events().map_while(|e| self.convert(e)).collect()
    }

    /// Converts an `orbclient::Event` to an `Event`.
    ///
    /// Returns none if the event cannot be converted.
    pub fn convert(&self, in_event: orbclient::Event) -> Option<Event> {
        match in_event.to_option() {
            orbclient::EventOption::Key(e) => {
                let text = if let Some(char) = convert_key(e.scancode) {
                    char
                } else {
                    e.character
                };

                if e.pressed {
                    Some(slint::platform::WindowEvent::KeyPressed { text }.into())
                } else {
                    Some(slint::platform::WindowEvent::KeyReleased { text }.into())
                }
            }

            orbclient::EventOption::Mouse(e) => {
                self.pointer_position
                    .set(slint::LogicalPosition::new(e.x as f32, e.y as f32));
                Some(
                    slint::platform::WindowEvent::PointerMoved {
                        position: self.pointer_position.get(),
                    }
                    .into(),
                )
            }
            orbclient::EventOption::Quit(_) => Some(Event::Quit),
            orbclient::EventOption::Resize(e) => Some(Event::Resize(slint::LogicalSize::new(
                e.width as f32,
                e.height as f32,
            ))),

            orbclient::EventOption::Button(e) => {
                let old_button_states = self.button_states.get();
                self.button_states
                    .set(ButtonStates::new(e.left, e.middle, e.right));

                let (button, pressed) = if old_button_states.left != self.button_states.get().left {
                    (
                        slint::platform::PointerEventButton::Left,
                        self.button_states.get().left,
                    )
                } else if old_button_states.middle != self.button_states.get().middle {
                    (
                        slint::platform::PointerEventButton::Middle,
                        self.button_states.get().middle,
                    )
                } else {
                    (
                        slint::platform::PointerEventButton::Right,
                        self.button_states.get().right,
                    )
                };

                if pressed {
                    Some(
                        slint::platform::WindowEvent::PointerPressed {
                            position: self.pointer_position.get(),
                            button,
                        }
                        .into(),
                    )
                } else {
                    Some(
                        slint::platform::WindowEvent::PointerReleased {
                            position: self.pointer_position.get(),
                            button,
                        }
                        .into(),
                    )
                }
            }
            orbclient::EventOption::Scroll(e) => Some(
                slint::platform::WindowEvent::PointerScrolled {
                    position: self.pointer_position.get(),
                    delta_x: e.x as f32,
                    delta_y: e.y as f32,
                }
                .into(),
            ),

            // will be handled maybe later
            orbclient::EventOption::Focus(_) => None,
            orbclient::EventOption::Move(_) => None,
            orbclient::EventOption::Screen(_) => None,
            orbclient::EventOption::Drop(_) => None,
            orbclient::EventOption::Hover(_) => None,

            // will not be handled
            orbclient::EventOption::Unknown(_) => None,
            orbclient::EventOption::None => None,
            orbclient::EventOption::MouseRelative(_) => None,

            // clipboard will read from Slint without updates
            orbclient::EventOption::Clipboard(_) => None,
            orbclient::EventOption::ClipboardUpdate(_) => None,

            // text input will be handled in Slint. only key events will be processed.
            orbclient::EventOption::TextInput(_) => None,
        }
    }
}

/// Converts an orbclient key scancode to a Slint key char representation.
pub fn convert_key(scancode: u8) -> Option<char> {
    match scancode {
        orbclient::K_TAB => Some(slint::platform::Key::Tab.into()),
        orbclient::K_CAPS => Some(slint::platform::Key::CapsLock.into()),
        orbclient::K_LEFT_SHIFT => Some(slint::platform::Key::Shift.into()),
        orbclient::K_RIGHT_SHIFT => Some(slint::platform::Key::ShiftR.into()),
        orbclient::K_CTRL => Some(slint::platform::Key::Control.into()),
        orbclient::K_ALT => Some(slint::platform::Key::Alt.into()),
        orbclient::K_ALT_GR => Some(slint::platform::Key::AltGr.into()),
        orbclient::K_ENTER => Some(slint::platform::Key::Return.into()),
        orbclient::K_ESC => Some(slint::platform::Key::Escape.into()),
        orbclient::K_F1 => Some(slint::platform::Key::F1.into()),
        orbclient::K_F2 => Some(slint::platform::Key::F2.into()),
        orbclient::K_F3 => Some(slint::platform::Key::F3.into()),
        orbclient::K_F4 => Some(slint::platform::Key::F4.into()),
        orbclient::K_F5 => Some(slint::platform::Key::F5.into()),
        orbclient::K_F6 => Some(slint::platform::Key::F6.into()),
        orbclient::K_F7 => Some(slint::platform::Key::F7.into()),
        orbclient::K_F8 => Some(slint::platform::Key::F8.into()),
        orbclient::K_F9 => Some(slint::platform::Key::F9.into()),
        orbclient::K_F10 => Some(slint::platform::Key::F10.into()),
        orbclient::K_F11 => Some(slint::platform::Key::F11.into()),
        orbclient::K_F12 => Some(slint::platform::Key::F12.into()),
        orbclient::K_UP => Some(slint::platform::Key::UpArrow.into()),
        orbclient::K_PGUP => Some(slint::platform::Key::PageUp.into()),
        orbclient::K_LEFT => Some(slint::platform::Key::LeftArrow.into()),
        orbclient::K_RIGHT => Some(slint::platform::Key::RightArrow.into()),
        orbclient::K_END => Some(slint::platform::Key::End.into()),
        orbclient::K_DOWN => Some(slint::platform::Key::DownArrow.into()),
        orbclient::K_PGDN => Some(slint::platform::Key::PageDown.into()),
        orbclient::K_DEL => Some(slint::platform::Key::Delete.into()),
        _ => None,
    }
}

#[cfg(test)]
mod test {
    pub use super::*;

    #[test]
    fn test_convert_key() {
        // Some
        assert_eq!(
            convert_key(orbclient::K_TAB),
            Some(slint::platform::Key::Tab.into())
        );
        assert_eq!(
            convert_key(orbclient::K_CAPS),
            Some(slint::platform::Key::CapsLock.into())
        );
        assert_eq!(
            convert_key(orbclient::K_LEFT_SHIFT),
            Some(slint::platform::Key::Shift.into())
        );
        assert_eq!(
            convert_key(orbclient::K_RIGHT_SHIFT),
            Some(slint::platform::Key::ShiftR.into())
        );
        assert_eq!(
            convert_key(orbclient::K_CTRL),
            Some(slint::platform::Key::Control.into())
        );
        assert_eq!(
            convert_key(orbclient::K_ALT),
            Some(slint::platform::Key::Alt.into())
        );
        assert_eq!(
            convert_key(orbclient::K_ALT_GR),
            Some(slint::platform::Key::AltGr.into())
        );
        assert_eq!(
            convert_key(orbclient::K_ENTER),
            Some(slint::platform::Key::Return.into())
        );
        assert_eq!(
            convert_key(orbclient::K_ESC),
            Some(slint::platform::Key::Escape.into())
        );
        assert_eq!(
            convert_key(orbclient::K_F1),
            Some(slint::platform::Key::F1.into())
        );
        assert_eq!(
            convert_key(orbclient::K_F2),
            Some(slint::platform::Key::F2.into())
        );
        assert_eq!(
            convert_key(orbclient::K_F3),
            Some(slint::platform::Key::F3.into())
        );
        assert_eq!(
            convert_key(orbclient::K_F4),
            Some(slint::platform::Key::F4.into())
        );
        assert_eq!(
            convert_key(orbclient::K_F5),
            Some(slint::platform::Key::F5.into())
        );
        assert_eq!(
            convert_key(orbclient::K_F6),
            Some(slint::platform::Key::F6.into())
        );
        assert_eq!(
            convert_key(orbclient::K_F7),
            Some(slint::platform::Key::F7.into())
        );
        assert_eq!(
            convert_key(orbclient::K_F8),
            Some(slint::platform::Key::F8.into())
        );
        assert_eq!(
            convert_key(orbclient::K_F9),
            Some(slint::platform::Key::F9.into())
        );
        assert_eq!(
            convert_key(orbclient::K_F10),
            Some(slint::platform::Key::F10.into())
        );
        assert_eq!(
            convert_key(orbclient::K_F11),
            Some(slint::platform::Key::F11.into())
        );
        assert_eq!(
            convert_key(orbclient::K_F12),
            Some(slint::platform::Key::F12.into())
        );
        assert_eq!(
            convert_key(orbclient::K_UP),
            Some(slint::platform::Key::UpArrow.into())
        );
        assert_eq!(
            convert_key(orbclient::K_PGUP),
            Some(slint::platform::Key::PageUp.into())
        );
        assert_eq!(
            convert_key(orbclient::K_LEFT),
            Some(slint::platform::Key::LeftArrow.into())
        );
        assert_eq!(
            convert_key(orbclient::K_RIGHT),
            Some(slint::platform::Key::RightArrow.into())
        );
        assert_eq!(
            convert_key(orbclient::K_END),
            Some(slint::platform::Key::End.into())
        );
        assert_eq!(
            convert_key(orbclient::K_DOWN),
            Some(slint::platform::Key::DownArrow.into())
        );
        assert_eq!(
            convert_key(orbclient::K_PGDN),
            Some(slint::platform::Key::PageDown.into())
        );
        assert_eq!(
            convert_key(orbclient::K_DEL),
            Some(slint::platform::Key::Delete.into())
        );

        // None
        assert_eq!(convert_key(orbclient::K_A), None);
    }

    #[test]
    fn test_key_event() {
        assert_eq!(
            EventReader::default()
                .convert(
                    orbclient::KeyEvent {
                        character: 'a',
                        scancode: orbclient::K_A,
                        pressed: false
                    }
                    .to_event()
                )
                .unwrap(),
            Event::WindowEvent(slint::platform::WindowEvent::KeyReleased { text: 'a' })
        );

        assert_eq!(
            EventReader::default()
                .convert(
                    orbclient::KeyEvent {
                        character: 'a',
                        scancode: orbclient::K_A,
                        pressed: true
                    }
                    .to_event()
                )
                .unwrap(),
            Event::WindowEvent(slint::platform::WindowEvent::KeyPressed { text: 'a' })
        );

        assert_eq!(
            EventReader::default()
                .convert(
                    orbclient::KeyEvent {
                        character: '0',
                        scancode: orbclient::K_TAB,
                        pressed: false
                    }
                    .to_event()
                )
                .unwrap(),
            Event::WindowEvent(slint::platform::WindowEvent::KeyReleased {
                text: slint::platform::Key::Tab.into()
            })
        );

        assert_eq!(
            EventReader::default()
                .convert(
                    orbclient::KeyEvent {
                        character: '0',
                        scancode: orbclient::K_TAB,
                        pressed: true
                    }
                    .to_event()
                )
                .unwrap(),
            Event::WindowEvent(slint::platform::WindowEvent::KeyPressed {
                text: slint::platform::Key::Tab.into()
            })
        );
    }

    #[test]
    fn test_mouse_event() {
        assert_eq!(
            EventReader::default()
                .convert(orbclient::MouseEvent { x: 10, y: 50 }.to_event())
                .unwrap(),
            Event::WindowEvent(slint::platform::WindowEvent::PointerMoved {
                position: slint::LogicalPosition::new(10.0, 50.0)
            })
        );
    }

    #[test]
    fn test_quit() {
        assert_eq!(
            EventReader::default()
                .convert(orbclient::QuitEvent.to_event())
                .unwrap(),
            Event::Quit
        );
    }

    #[test]
    fn test_resize() {
        assert_eq!(
            EventReader::default()
                .convert(
                    orbclient::ResizeEvent {
                        width: 40,
                        height: 50
                    }
                    .to_event()
                )
                .unwrap(),
            Event::Resize(slint::LogicalSize::new(40., 50.))
        );
    }

    #[test]
    fn test_scroll() {
        assert_eq!(
            EventReader::default()
                .convert(orbclient::ScrollEvent { x: 40, y: 50 }.to_event())
                .unwrap(),
            Event::WindowEvent(slint::platform::WindowEvent::PointerScrolled {
                delta_x: 40.,
                delta_y: 50.,
                position: slint::LogicalPosition::new(0., 0.)
            })
        );
    }

    #[test]
    fn test_button() {
        let event_reader = EventReader::default();

        assert_eq!(
            event_reader
                .convert(
                    orbclient::ButtonEvent {
                        left: true,
                        right: false,
                        middle: false
                    }
                    .to_event()
                )
                .unwrap(),
            Event::WindowEvent(slint::platform::WindowEvent::PointerPressed {
                button: slint::platform::PointerEventButton::Left,
                position: slint::LogicalPosition::new(0., 0.)
            })
        );

        assert_eq!(
            event_reader
                .convert(
                    orbclient::ButtonEvent {
                        left: false,
                        right: false,
                        middle: false
                    }
                    .to_event()
                )
                .unwrap(),
            Event::WindowEvent(slint::platform::WindowEvent::PointerReleased {
                button: slint::platform::PointerEventButton::Left,
                position: slint::LogicalPosition::new(0., 0.)
            })
        );
    }
}
