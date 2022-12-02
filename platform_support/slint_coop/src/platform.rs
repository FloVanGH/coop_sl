// SPDX-FileCopyrightText: 2022 Florian Blasius <co_sl@tutanota.com>
// SPDX-License-Identifier: MIT

use std::cell::*;
use std::rc::*;

use slint::platform::software_renderer as renderer;

use crate::Color;

/// Initializes the platform.
pub fn init() {
    slint::platform::set_platform(Box::new(CoopPlatform::default())).unwrap();
}

/// Initializes the platform with a configuration.
pub fn init_config(width: f32, height: f32, title: impl Into<String>) {
    slint::platform::set_platform(Box::new(CoopPlatform::new(width, height, title.into())))
        .unwrap();
}

/// Slint platform implementation based on `coop_client`.
pub struct CoopPlatform {
    slint_window: RefCell<Rc<renderer::MinimalSoftwareWindow<1>>>,
    coop_window: RefCell<coop_client::Window>,
}

impl CoopPlatform {
    /// Returns a new platform object.
    pub fn new(width: f32, height: f32, title: String) -> Self {
        Self {
            slint_window: RefCell::new(renderer::MinimalSoftwareWindow::new()),
            coop_window: RefCell::new(coop_client::Window::new(width, height, title, true)),
        }
    }
}

impl Default for CoopPlatform {
    fn default() -> Self {
        CoopPlatform::new(0., 0., String::default())
    }
}

// todo: event converter with tests

impl slint::platform::Platform for CoopPlatform {
    fn create_window_adapter(&self) -> Rc<dyn slint::platform::WindowAdapter> {
        self.slint_window.borrow().clone()
    }

    fn run_event_loop(&self) {
        let (width, height) = self.coop_window.borrow().size();

        self.slint_window
            .borrow()
            .as_ref()
            .set_size(slint::PhysicalSize::new(width as u32, height as u32));

        let mut work_buffer = vec![Color(0); width as usize * height as usize];

        'events: loop {
            slint::platform::update_timers_and_animations();
            self.slint_window.borrow().draw_if_needed(|renderer| {
                renderer.render(
                    &mut work_buffer,
                    self.slint_window.borrow().size().width as usize,
                );

                let len = work_buffer.len() * std::mem::size_of::<Color>();

                let buffer =
                    unsafe { std::slice::from_raw_parts(work_buffer.as_ptr() as *mut u8, len) };

                self.coop_window.borrow().draw(buffer, width, height);
            });

            for event in crate::read(&self.coop_window.borrow()) {
                match event {
                    crate::Event::WindowEvent(e) => self.slint_window.borrow().dispatch_event(e),
                    crate::Event::Resize(_) => todo!(),
                    crate::Event::Quit => {
                        self.coop_window.borrow().close();
                        break 'events;
                    }
                }
            }
        }
    }
}
