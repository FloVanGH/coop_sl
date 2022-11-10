// SPDX-FileCopyrightText: 2022 Florian Blasius <co_sl@tutanota.com>
// SPDX-License-Identifier: MIT

use psp::sys::*;

/// Describes an event from the PSP.
#[derive(Copy, Clone)]
pub enum PspEvent {
    ButtonPressed { button: CtrlButtons },
    ButtonReleased { button: CtrlButtons },
    StickMoved { x: u8, y: u8 },
}

/// Used to iterator over a list of  `PspEvent`s.
pub struct PspEventIter {
    events: alloc::vec::Vec<PspEvent>,
    i: usize,
}

impl Iterator for PspEventIter {
    type Item = PspEvent;

    fn next(&mut self) -> Option<PspEvent> {
        if self.i < self.events.len() {
            if let Some(event) = self.events.get(self.i) {
                self.i += 1;
                Some(*event)
            } else {
                None
            }
        } else {
            None
        }
    }
}

/// The `PspEventReader` is used to read events from the buffer of the PSP.
#[derive(Default, Clone)]
pub struct PspEventReader {
    data: core::cell::Cell<SceCtrlData>,
}

impl PspEventReader {
    /// Reads and returns the events of the PSP as iterator.
    pub fn events(&self) -> PspEventIter {
        let mut events = alloc::vec::Vec::new();

        let mut new_data = SceCtrlData::default();

        unsafe {
            // Read button/analog input
            sceCtrlReadBufferPositive(&mut new_data, 1);
        };

        // System
        self.check(CtrlButtons::SELECT, &mut events, &new_data);
        self.check(CtrlButtons::START, &mut events, &new_data);
        self.check(CtrlButtons::HOME, &mut events, &new_data);

        // D-Pad
        self.check(CtrlButtons::LEFT, &mut events, &new_data);
        self.check(CtrlButtons::UP, &mut events, &new_data);
        self.check(CtrlButtons::RIGHT, &mut events, &new_data);
        self.check(CtrlButtons::DOWN, &mut events, &new_data);

        // Trigger
        self.check(CtrlButtons::LTRIGGER, &mut events, &new_data);
        self.check(CtrlButtons::RTRIGGER, &mut events, &new_data);

        // Game
        self.check(CtrlButtons::TRIANGLE, &mut events, &new_data);
        self.check(CtrlButtons::SQUARE, &mut events, &new_data);
        self.check(CtrlButtons::CIRCLE, &mut events, &new_data);
        self.check(CtrlButtons::CROSS, &mut events, &new_data);

        if new_data.lx != 0 || new_data.ly != 0 {
            events.push(PspEvent::StickMoved {
                x: new_data.lx,
                y: new_data.ly,
            });
        }

        self.data.set(new_data);
        PspEventIter { events, i: 0 }
    }

    fn check(
        &self,
        button: CtrlButtons,
        events: &mut alloc::vec::Vec<PspEvent>,
        new_data: &SceCtrlData,
    ) {
        if new_data.buttons.contains(button) {
            events.push(PspEvent::ButtonPressed { button })
        }

        if self.data.get().buttons.contains(button) && !new_data.buttons.contains(button) {
            events.push(PspEvent::ButtonReleased { button })
        }
    }
}
