// SPDX-FileCopyrightText: 2022 Florian Blasius <co_sl@tutanota.com>
// SPDX-License-Identifier: MIT

use alloc::vec::*;
use core::cell::*;
use psp::sys::*;

/// Reads in events from the psp system data and converts them into Slint `WindowEvents`s.
#[derive(Debug, Default, Clone)]
pub struct EventReader {
    current_data: core::cell::Cell<SceCtrlData>,
    pointer_position: Cell<slint::LogicalPosition>,
}

impl EventReader {
    /// Reads all events from the given psp system data and converts them into a vec of `slint::platform::WindowEvent`.
    pub fn read(&self, new_data: &SceCtrlData) -> Vec<slint::platform::WindowEvent> {
        let mut events = Vec::new();

        // map right button to tab
        if let Some(pressed) = self.button_state(new_data, &CtrlButtons::RIGHT) {
            if pressed {
                events.push(slint::platform::WindowEvent::KeyPressed {
                    text: slint::platform::Key::Tab.into(),
                });
            } else {
                events.push(slint::platform::WindowEvent::KeyReleased {
                    text: slint::platform::Key::Tab.into(),
                });
            }
        }

        // map left button to back tab
        if let Some(pressed) = self.button_state(new_data, &CtrlButtons::LEFT) {
            if pressed {
                // for back tab we need also shift
                events.push(slint::platform::WindowEvent::KeyPressed {
                    text: slint::platform::Key::Shift.into(),
                });
                events.push(slint::platform::WindowEvent::KeyPressed {
                    text: slint::platform::Key::Tab.into(),
                });
            } else {
                // for back tab we need also shift
                events.push(slint::platform::WindowEvent::KeyReleased {
                    text: slint::platform::Key::Shift.into(),
                });
                events.push(slint::platform::WindowEvent::KeyReleased {
                    text: slint::platform::Key::Tab.into(),
                });
            }
        }

        // map cross button to return
        if let Some(pressed) = self.button_state(new_data, &CtrlButtons::CROSS) {
            if pressed {
                events.push(slint::platform::WindowEvent::KeyPressed {
                    text: slint::platform::Key::Return.into(),
                });
            } else {
                events.push(slint::platform::WindowEvent::KeyReleased {
                    text: slint::platform::Key::Return.into(),
                });
            }
        }

        // map circle button to escape
        if let Some(pressed) = self.button_state(new_data, &CtrlButtons::CIRCLE) {
            if pressed {
                events.push(slint::platform::WindowEvent::KeyPressed {
                    text: slint::platform::Key::Escape.into(),
                });
            } else {
                events.push(slint::platform::WindowEvent::KeyReleased {
                    text: slint::platform::Key::Escape.into(),
                });
            }
        }

        // map square button to left pointer button
        if let Some(pressed) = self.button_state(&new_data, &CtrlButtons::SQUARE) {
            if pressed {
                events.push(slint::platform::WindowEvent::PointerPressed {
                    position: self.pointer_position.get(),
                    button: slint::platform::PointerEventButton::Left,
                });
            } else {
                events.push(slint::platform::WindowEvent::PointerReleased {
                    position: self.pointer_position.get(),
                    button: slint::platform::PointerEventButton::Left,
                });
            }
        }

        // stick move to pointer move
        // if new_data.lx != 0 || new_data.ly != 0 {
        //     let mut position = self.pointer_position.get();
        //     // (* 3.) to make the movement faster
        //     position.x += new_data.lx as f32 * 3.;
        //     position.y += new_data.ly as f32 * 3.;

        //     events.push(slint::platform::WindowEvent::PointerMoved { position });
        //     self.pointer_position.set(position);
        // }

        // map up to scroll up and down to scroll down
        if new_data.buttons.contains(CtrlButtons::UP) {
            events.push(slint::platform::WindowEvent::PointerScrolled {
                position: self.pointer_position.get(),
                delta_x: 0.,
                delta_y: 1.,
            });
        }

        if new_data.buttons.contains(CtrlButtons::DOWN) {
            events.push(slint::platform::WindowEvent::PointerScrolled {
                position: self.pointer_position.get(),
                delta_x: 0.,
                delta_y: -1.,
            });
        }

        self.current_data.set(*new_data);

        events
    }

    fn button_state(&self, new_data: &SceCtrlData, psp_button: &CtrlButtons) -> Option<bool> {
        if new_data.buttons.contains(*psp_button) && !self.current_data.get().buttons.contains(*psp_button) {
            Some(true)
        } else if !new_data.buttons.contains(*psp_button)
            && self.current_data.get().buttons.contains(*psp_button)
        {
            Some(false)
        } else {
            None
        }
    }
}
