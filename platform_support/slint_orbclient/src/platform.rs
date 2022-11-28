// SPDX-FileCopyrightText: 2022 Florian Blasius <co_sl@tutanota.com>
// SPDX-License-Identifier: MIT

use std::cell::*;
use std::rc::*;

use orbclient::{Renderer, Window};

use crate::color::*;
use crate::event_reader::*;

static INITIAL_INSTANT: once_cell::sync::OnceCell<instant::Instant> =
    once_cell::sync::OnceCell::new();

/// Used to configure the platform window.
#[derive(Clone, Default, Debug)]
pub struct Config {
    x: i32,
    y: i32,
    width: u32,
    height: u32,
    title: String,
    events_async: bool,
    resizable: bool,
}

impl Config {
    /// Sets the x position of the window.
    pub fn x(mut self, x: i32) -> Self {
        self.x = x;
        self
    }

    /// Sets the y position of the window.
    pub fn y(mut self, y: i32) -> Self {
        self.y = y;
        self
    }

    /// Sets the width of the window.
    pub fn width(mut self, width: u32) -> Self {
        self.width = width;
        self
    }

    /// Sets the width of the window.
    pub fn height(mut self, height: u32) -> Self {
        self.height = height;
        self
    }

    /// Sets the title of the window.
    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.title = title.into();
        self
    }

    /// If set to `false` the event reading will blocking the window loop until an events occurs.
    pub fn events_async(mut self, events_async: bool) -> Self {
        self.events_async = events_async;
        self
    }

    /// If set to `true` the window is resizable.
    pub fn resizable(mut self, resizable: bool) -> Self {
        self.resizable = resizable;
        self
    }
}

impl From<Config> for OrbClientPlatform {
    fn from(config: Config) -> Self {
        let mut flags = vec![];

        if config.events_async {
            flags.push(orbclient::WindowFlag::Async);
        }

        if config.resizable {
            flags.push(orbclient::WindowFlag::Resizable);
        }

        Self {
            slint_window: core::cell::RefCell::new(
                slint::platform::software_renderer::MinimalSoftwareWindow::new(),
            ),
            orb_window: RefCell::new(
                Window::new_flags(
                    config.x,
                    config.y,
                    config.width,
                    config.height,
                    &config.title,
                    &flags,
                )
                .unwrap(),
            ),
            event_reader: EventReader::default(),
        }
    }
}

/// Describes a `Slint` platform that is based on `OrbClient` and so can run on also on `Redox OS`.
pub struct OrbClientPlatform {
    slint_window: RefCell<Rc<slint::platform::software_renderer::MinimalSoftwareWindow<1>>>,
    orb_window: RefCell<Window>,
    event_reader: EventReader,
}

impl OrbClientPlatform {
    /// Returns a new platform object.
    pub fn new() -> Self {
        Config::default()
            .width(600)
            .height(400)
            .title("Slint orbclient window")
            .into()
    }
}

impl Default for OrbClientPlatform {
    fn default() -> Self {
        OrbClientPlatform::new()
    }
}

// todo: event converter with tests

impl slint::platform::Platform for OrbClientPlatform {
    fn create_window_adapter(&self) -> Rc<dyn slint::platform::WindowAdapter> {
        self.slint_window.borrow().clone()
    }

    fn set_clipboard_text(&self, text: &str) {
        self.orb_window.borrow_mut().set_clipboard(text);
    }

    fn clipboard_text(&self) -> Option<String> {
        Some(self.orb_window.borrow().clipboard())
    }

    fn duration_since_start(&self) -> core::time::Duration {
        let the_beginning = *INITIAL_INSTANT.get_or_init(instant::Instant::now);
        instant::Instant::now() - the_beginning
    }

    fn run_event_loop(&self) {
        let width = self.orb_window.borrow().width();
        let height = self.orb_window.borrow().height();

        self.slint_window
            .borrow()
            .as_ref()
            .set_size(slint::PhysicalSize::new(width, height));

        let mut work_buffer = vec![Color(0); width as usize * height as usize];

        'events: loop {
            slint::platform::update_timers_and_animations();
            self.slint_window.borrow().draw_if_needed(|renderer| {
                renderer.render(
                    &mut work_buffer,
                    self.slint_window.borrow().size().width as usize,
                );

                unsafe {
                    core::ptr::copy(
                        work_buffer.as_ptr(),
                        self.orb_window.borrow_mut().data_mut().as_mut_ptr() as *mut Color,
                        work_buffer.len(),
                    );
                };

                self.orb_window.borrow_mut().sync();
            });

            for event in self.event_reader.read(&mut self.orb_window.borrow_mut()) {
                match event {
                    Event::WindowEvent(e) => self.slint_window.borrow().dispatch_event(e),
                    Event::Resize(s) => {
                        self.slint_window.borrow().set_size(s);

                        work_buffer.resize(s.width as usize * s.height as usize, Color(0));
                    }
                    Event::Quit => break 'events,
                }
            }
        }
    }
}
