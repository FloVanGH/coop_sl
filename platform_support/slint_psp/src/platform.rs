// SPDX-FileCopyrightText: 2022 Florian Blasius <co_sl@tutanota.com>
// SPDX-License-Identifier: MIT

use alloc::rc::*;
use core::cell::*;

use psp::sys::*;

use crate::color::*;
use crate::events::*;

/// Initializes the psp `Slint` backend.
pub fn init() {
    slint::platform::set_platform(alloc::boxed::Box::new(PspBackend::new())).unwrap();
}

/// Describes a `Slint` backend for the PSP.
pub struct PspBackend {
    window: RefCell<Rc<slint::platform::software_renderer::MinimalSoftwareWindow<1>>>,
    event_reader: PspEventReader,
    pointer_position: RefCell<slint::LogicalPosition>,
}

impl PspBackend {
    /// Returns a new platform object.
    pub fn new() -> Self {
        Self {
            window: core::cell::RefCell::new(
                slint::platform::software_renderer::MinimalSoftwareWindow::new(),
            ),
            event_reader: PspEventReader::default(),
            pointer_position: RefCell::new(slint::LogicalPosition::default()),
        }
    }
}

impl Default for PspBackend {
    fn default() -> Self {
        PspBackend::new()
    }
}

impl PspBackend {
    fn move_pointer(&self, offset: slint::LogicalPosition) {
        if offset.x == 0. && offset.y == 0. {
            return;
        }

        self.pointer_position.borrow_mut().x += offset.x * 3.;
        self.pointer_position.borrow_mut().y += offset.y * 3.;

        self.window
            .borrow()
            .dispatch_event(slint::WindowEvent::PointerMoved {
                position: *self.pointer_position.borrow(),
            });
    }
}

impl slint::platform::Platform for PspBackend {
    fn create_window_adapter(&self) -> Rc<dyn slint::platform::WindowAdapter> {
        self.window.borrow().clone()
    }

    fn duration_since_start(&self) -> core::time::Duration {
        let mut tick = 0;
        unsafe {
            sceRtcGetCurrentTick(&mut tick);
        }
        core::time::Duration::from_micros(tick)
    }

    fn run_event_loop(&self) {
        #[link_section = ".frame_buffer"]
        static mut BUFFER: [Color; psp::BUF_WIDTH as usize * psp::SCREEN_HEIGHT as usize] =
            [Color(0); psp::BUF_WIDTH as usize * psp::SCREEN_HEIGHT as usize];

        // SAFETY the init function is only called once (as enforced by Peripherals::take)
        let buffer = unsafe { &mut BUFFER };

        self.window
            .borrow()
            .as_ref()
            .set_size(slint::PhysicalSize::new(
                psp::SCREEN_WIDTH,
                psp::SCREEN_HEIGHT,
            ));

        // -------- Setup the paps frame buffer --------
        let vram_base = unsafe {
            sceDisplaySetMode(
                DisplayMode::Lcd,
                psp::SCREEN_WIDTH as usize,
                psp::SCREEN_HEIGHT as usize,
            );
            let vram_base = (0x4000_0000u32 | sceGeEdramGetAddr() as u32) as *mut Color;
            sceDisplaySetFrameBuf(
                vram_base as *const u8,
                psp::BUF_WIDTH as usize,
                DisplayPixelFormat::Psm8888,
                DisplaySetBufSync::NextFrame,
            );

            vram_base
        };

        let work_buffer: &mut [Color] = buffer;

        loop {
            slint::platform::update_timers_and_animations();
            self.window.borrow().draw_if_needed(|renderer| {
                renderer.render(work_buffer, psp::BUF_WIDTH as usize);

                unsafe { core::ptr::copy(work_buffer.as_ptr(), vram_base, work_buffer.len()) };
            });

            for event in self.event_reader.events() {
                match event {
                    PspEvent::ButtonPressed { button } => match button {
                        CtrlButtons::LEFT => {
                            self.move_pointer(slint::LogicalPosition::new(-1., 0.))
                        }
                        CtrlButtons::UP => self.move_pointer(slint::LogicalPosition::new(0., -1.)),
                        CtrlButtons::RIGHT => {
                            self.move_pointer(slint::LogicalPosition::new(1., 0.))
                        }
                        CtrlButtons::DOWN => self.move_pointer(slint::LogicalPosition::new(0., 1.)),
                        CtrlButtons::CROSS => {
                            self.window.borrow().dispatch_event(
                                slint::WindowEvent::PointerPressed {
                                    position: *self.pointer_position.borrow(),
                                    button: slint::PointerEventButton::Left,
                                },
                            );
                        }
                        _ => {}
                    },
                    PspEvent::ButtonReleased { button } => {
                        if button.eq(&CtrlButtons::CROSS) {
                            self.window.borrow().dispatch_event(
                                slint::WindowEvent::PointerReleased {
                                    position: *self.pointer_position.borrow(),
                                    button: slint::PointerEventButton::Left,
                                },
                            );
                        }
                    }
                    _ => {} // PspEvent::StickMoved { x, y } => {
                            //     self.move_pointer(slint::LogicalPosition::new(x as f32, y as f32));
                            // }
                }
            }
        }
    }
}
