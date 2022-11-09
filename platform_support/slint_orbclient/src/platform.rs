use std::cell::*;
use std::rc::*;

use orbclient::{EventOption, Renderer, Window};

use crate::color::*;

use slint::LogicalSize;

static INITIAL_INSTANT: once_cell::sync::OnceCell<instant::Instant> =
    once_cell::sync::OnceCell::new();

/// Initializes the platform.
pub fn init() {
    slint::platform::set_platform(Box::new(OrbClientPlatform::new())).unwrap();
}

/// Describes a `Slint` platform that is based on `OrbClient` and so can run on also on `Redox OS`
pub struct OrbClientPlatform {
    slint_window: RefCell<Rc<slint::platform::software_renderer::MinimalSoftwareWindow<1>>>,
    orb_window: RefCell<Window>,
    button_states: Cell<(bool, bool, bool)>,
    pointer_position: Cell<slint::LogicalPosition>,
}

impl OrbClientPlatform {
    /// Returns a new platform object.
    pub fn new() -> Self {
        Self {
            slint_window: core::cell::RefCell::new(
                slint::platform::software_renderer::MinimalSoftwareWindow::new(),
            ),
            orb_window: RefCell::new(
                Window::new_flags(
                    0,
                    0,
                    600,
                    320,
                    "Slint window",
                    &[
                        orbclient::WindowFlag::Resizable,
                        orbclient::WindowFlag::Async,
                    ],
                )
                .unwrap(),
            ),
            pointer_position: Cell::new(slint::LogicalPosition::default()),
            button_states: Cell::new((false, false, false)),
        }
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

            for event in self.orb_window.borrow_mut().events() {
                match event.to_option() {
                    EventOption::Quit(_quit_event) => break 'events,
                    EventOption::Resize(resize) => {
                        self.slint_window
                            .borrow()
                            .set_size(LogicalSize::new(resize.width as f32, resize.height as f32));

                        work_buffer
                            .resize(resize.width as usize * resize.height as usize, Color(0));
                    }

                    EventOption::Mouse(evt) => {
                        self.pointer_position
                            .set(slint::LogicalPosition::new(evt.x as f32, evt.y as f32));
                        self.slint_window.borrow().dispatch_event(
                            slint::WindowEvent::PointerMoved {
                                position: self.pointer_position.get(),
                            },
                        );
                    }

                    EventOption::Button(button) => {
                        if self.button_states.get().0 != button.left {
                            if button.left {
                                self.slint_window.borrow().dispatch_event(
                                    slint::WindowEvent::PointerPressed {
                                        position: self.pointer_position.get(),
                                        button: slint::PointerEventButton::Left,
                                    },
                                );
                            } else {
                                self.slint_window.borrow().dispatch_event(
                                    slint::WindowEvent::PointerReleased {
                                        position: self.pointer_position.get(),
                                        button: slint::PointerEventButton::Left,
                                    },
                                );
                            }
                        }

                        if self.button_states.get().1 != button.middle {
                            if button.middle {
                                self.slint_window.borrow().dispatch_event(
                                    slint::WindowEvent::PointerPressed {
                                        position: self.pointer_position.get(),
                                        button: slint::PointerEventButton::Middle,
                                    },
                                );
                            } else {
                                self.slint_window.borrow().dispatch_event(
                                    slint::WindowEvent::PointerReleased {
                                        position: self.pointer_position.get(),
                                        button: slint::PointerEventButton::Middle,
                                    },
                                );
                            }
                        }

                        if self.button_states.get().2 != button.right {
                            if button.right {
                                self.slint_window.borrow().dispatch_event(
                                    slint::WindowEvent::PointerPressed {
                                        position: self.pointer_position.get(),
                                        button: slint::PointerEventButton::Right,
                                    },
                                );
                            } else {
                                self.slint_window.borrow().dispatch_event(
                                    slint::WindowEvent::PointerReleased {
                                        position: self.pointer_position.get(),
                                        button: slint::PointerEventButton::Right,
                                    },
                                );
                            }
                        }

                        self.button_states
                            .set((button.left, button.middle, button.right));
                    }
                    EventOption::Scroll(scroll) => {
                        self.slint_window.borrow().dispatch_event(
                            slint::WindowEvent::PointerScrolled {
                                position: self.pointer_position.get(),
                                delta_x: scroll.x as f32 * 2.0,
                                delta_y: scroll.y as f32 * 2.0,
                            },
                        );
                    }
                    _ => {}
                }
            }
        }
    }
}
