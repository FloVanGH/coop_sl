use std::cell::*;
use std::rc::*;

use orbclient::{Renderer, Window};

use crate::color::*;

pub fn init() {
    slint::platform::set_platform(Box::new(RedoxPlatform::new())).unwrap();
}

/// Describes a `Slint` platform that must be used to run `Slint` on the PSP.
pub struct RedoxPlatform {
    window: RefCell<Rc<slint::platform::software_renderer::MinimalSoftwareWindow<1>>>,
    orb_window: RefCell<Window>,
    _pointer_position: RefCell<slint::LogicalPosition>,
}

impl RedoxPlatform {
    /// Returns a new platform object.
    pub fn new() -> Self {
        Self {
            window: core::cell::RefCell::new(
                slint::platform::software_renderer::MinimalSoftwareWindow::new(),
            ),
            orb_window: RefCell::new(Window::new(0, 0, 600, 320, "Slint window").unwrap()),
            _pointer_position: RefCell::new(slint::LogicalPosition::default()),
        }
    }
}

impl Default for RedoxPlatform {
    fn default() -> Self {
        RedoxPlatform::new()
    }
}

impl RedoxPlatform {
    fn move_pointer(&self, offset: slint::LogicalPosition) {
        if offset.x == 0. && offset.y == 0. {
            return;
        }

        self._pointer_position.borrow_mut().x += offset.x * 3.;
        self._pointer_position.borrow_mut().y += offset.y * 3.;

        self.window
            .borrow()
            .dispatch_event(slint::WindowEvent::PointerMoved {
                position: *self._pointer_position.borrow(),
            });
    }
}

impl slint::platform::Platform for RedoxPlatform {
    fn create_window_adapter(&self) -> Rc<dyn slint::platform::WindowAdapter> {
        self.window.borrow().clone()
    }

    fn run_event_loop(&self) {
        // #[link_section = ".frame_buffer"]
        // static mut BUFFER: [Color; psp::BUF_WIDTH as usize * psp::SCREEN_HEIGHT as usize] =
        //     [Color(0); psp::BUF_WIDTH as usize * psp::SCREEN_HEIGHT as usize];

        // // SAFETY the init function is only called once (as enforced by Peripherals::take)
        // let buffer = unsafe { &mut BUFFER };

        let width = self.orb_window.borrow().width();
        let height = self.orb_window.borrow().height();

        self.window
            .borrow()
            .as_ref()
            .set_size(slint::PhysicalSize::new(width, height));

        // let work_buffer: &mut [Color] = buffer;
        let mut work_buffer = vec![Color(0); width as usize * height as usize];

        loop {
            slint::platform::update_timers_and_animations();
            self.window.borrow().draw_if_needed(|renderer| {
                renderer.render(&mut work_buffer, width as usize);

                unsafe {
                    core::ptr::copy(
                        work_buffer.as_ptr(),
                        self.orb_window.borrow_mut().data_mut().as_mut_ptr() as *mut Color,
                        work_buffer.len(),
                    )
                };
            });

            // todo: event reading
        }
    }
}
