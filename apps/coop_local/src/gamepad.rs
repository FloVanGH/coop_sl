// SPDX-FileCopyrightText: 2023 Florian Blasius <co_sl@tutanota.com>
// SPDX-License-Identifier: GPL-3.0-only

use crate::ui::MainWindow;
use gilrs::{Button, Event, EventType, Gilrs};
use slint::{platform::Key, *};

pub fn connect(view_handle: Weak<MainWindow>) -> Result<Timer, PlatformError> {
    let mut gilrs = Gilrs::new().map_err(|e| PlatformError::from(e.to_string()))?;

    let update_timer = Timer::default();

    update_timer.start(
        TimerMode::Repeated,
        std::time::Duration::from_millis(200),
        move || {
            if let Some(main_window) = view_handle.upgrade() {
                while let Some(event) = gilrs.next_event() {
                    let Event { event, .. } = event;

                    match event {
                        EventType::ButtonPressed(button, _) => {
                            dispatch_key_pressed(button, main_window.window())
                        }
                        EventType::ButtonReleased(button, _) => {
                            dispatch_key_released(button, main_window.window())
                        }
                        _ => (),
                    }
                }
            }
        },
    );

    Ok(update_timer)
}

fn key_from_button(button: Button) -> SharedString {
    match button {
        Button::South => Key::Return.into(),
        Button::East => Key::Escape.into(),
        Button::RightTrigger => Key::Tab.into(),
        Button::North => " ".into(),
        Button::DPadUp => Key::UpArrow.into(),
        Button::DPadDown => Key::DownArrow.into(),
        Button::DPadLeft => Key::LeftArrow.into(),
        Button::DPadRight => Key::RightArrow.into(),
        _ => SharedString::default(),
    }
}

fn dispatch_key_pressed(button: Button, window: &Window) {
    let key = key_from_button(button);

    if key.is_empty() {
        return;
    }

    window.dispatch_event(platform::WindowEvent::KeyPressed { text: key });
}

fn dispatch_key_released(button: Button, window: &Window) {
    let key = key_from_button(button);

    if key.is_empty() {
        return;
    }

    window.dispatch_event(platform::WindowEvent::KeyReleased { text: key });
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_key_from_button() {
        assert_eq!(
            key_from_button(Button::South),
            SharedString::from(Key::Return)
        );
        assert_eq!(
            key_from_button(Button::East),
            SharedString::from(Key::Escape)
        );
        assert_eq!(
            key_from_button(Button::RightTrigger),
            SharedString::from(Key::Tab)
        );
        assert_eq!(key_from_button(Button::North), SharedString::from(" "));
        assert_eq!(
            key_from_button(Button::DPadUp),
            SharedString::from(Key::UpArrow)
        );
        assert_eq!(
            key_from_button(Button::DPadDown),
            SharedString::from(Key::DownArrow)
        );
        assert_eq!(
            key_from_button(Button::DPadLeft),
            SharedString::from(Key::LeftArrow)
        );
        assert_eq!(
            key_from_button(Button::DPadRight),
            SharedString::from(Key::RightArrow)
        );

        assert_eq!(key_from_button(Button::West), SharedString::default());
        assert_eq!(key_from_button(Button::C), SharedString::default());
        assert_eq!(key_from_button(Button::Z), SharedString::default());
        assert_eq!(
            key_from_button(Button::LeftTrigger),
            SharedString::default()
        );
        assert_eq!(
            key_from_button(Button::LeftTrigger2),
            SharedString::default()
        );
        assert_eq!(
            key_from_button(Button::RightTrigger2),
            SharedString::default()
        );
        assert_eq!(key_from_button(Button::Select), SharedString::default());
        assert_eq!(key_from_button(Button::Start), SharedString::default());
        assert_eq!(key_from_button(Button::Mode), SharedString::default());
        assert_eq!(key_from_button(Button::LeftThumb), SharedString::default());
        assert_eq!(key_from_button(Button::RightThumb), SharedString::default());
        assert_eq!(key_from_button(Button::Unknown), SharedString::default());
    }
}
