// SPDX-FileCopyrightText: 2022 Florian Blasius <co_sl@tutanota.com>
// SPDX-License-Identifier: MIT

use alloc::rc::*;
use core::cell::*;
use slint::LogicalPosition;

use psp::sys::*;

use crate::color::*;
use crate::event_reader::*;

/// Describes a `Slint` platform implementation for the PSP.
pub struct PspPlatform {
    slint_window: RefCell<Rc<slint::platform::software_renderer::MinimalSoftwareWindow<1>>>,
    event_reader: EventReader,
    show_pointer: bool,
}

impl PspPlatform {
    /// Returns a new platform object.
    pub fn new(show_pointer: bool) -> Self {
        Self {
            slint_window: core::cell::RefCell::new(
                slint::platform::software_renderer::MinimalSoftwareWindow::new(),
            ),
            event_reader: EventReader::default(),
            show_pointer: show_pointer,
        }
    }
}

impl Default for PspPlatform {
    fn default() -> Self {
        Self {
            slint_window: core::cell::RefCell::new(
                slint::platform::software_renderer::MinimalSoftwareWindow::new(),
            ),
            event_reader: EventReader::default(),
            show_pointer: false,
        }
    }
}

impl slint::platform::Platform for PspPlatform {
    fn create_window_adapter(&self) -> Rc<dyn slint::platform::WindowAdapter> {
        self.slint_window.borrow().clone()
    }

    fn duration_since_start(&self) -> core::time::Duration {
        let mut tick = 0;
        unsafe {
            sceRtcGetCurrentTick(&mut tick);
        }
        core::time::Duration::from_micros(tick)
    }

    fn run_event_loop(&self) {
        use embedded_graphics::{
            pixelcolor::Rgb888,
            prelude::*,
            primitives::{Circle, PrimitiveStyleBuilder},
        };

        #[link_section = ".frame_buffer"]
        static mut BUFFER: [Color; psp::BUF_WIDTH as usize * psp::SCREEN_HEIGHT as usize] =
            [Color(0); psp::BUF_WIDTH as usize * psp::SCREEN_HEIGHT as usize];

        // SAFETY the init function is only called once (as enforced by Peripherals::take)
        let buffer = unsafe { &mut BUFFER };

        self.slint_window
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
        let mut pointer_position = LogicalPosition::default();
        let mut new_data = SceCtrlData::default();

        loop {
            slint::platform::update_timers_and_animations();
            self.slint_window.borrow().draw_if_needed(|renderer| {
                renderer.render(work_buffer, psp::BUF_WIDTH as usize);

                // draw pointer
                if self.show_pointer {
                    Circle::new(
                        Point::new(pointer_position.x as i32, pointer_position.y as i32),
                        4,
                    )
                    .into_styled(
                        PrimitiveStyleBuilder::new()
                            .stroke_width(1)
                            .stroke_color(Rgb888::BLACK)
                            .fill_color(Rgb888::WHITE)
                            .build(),
                    )
                    .draw(&mut PspFrameBuffer::new(work_buffer))
                    .expect("Cannot draw pointer");
                }

                unsafe { core::ptr::copy(work_buffer.as_ptr(), vram_base, work_buffer.len()) };
            });

            unsafe {
                // Read button/analog input
                sceCtrlReadBufferPositive(&mut new_data, 1);
            };

            for event in self.event_reader.read(&new_data) {
                if let slint::platform::WindowEvent::PointerMoved { position } = event {
                    pointer_position = position;
                }
                
                self.slint_window.borrow().dispatch_event(event);
            }
        }
    }
}
