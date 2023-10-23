// SPDX-FileCopyrightText: 2022 Florian Blasius <co_sl@tutanota.com>
// SPDX-License-Identifier: MIT

#![cfg_attr(feature = "mcu-board-support", no_std)]
#![cfg_attr(feature = "mcu-board-support", no_main)]

extern crate alloc;

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

#[allow(clippy::all)]
mod generated_code {
    #![allow(dead_code)]
    slint::include_modules!();
}

use alloc::rc::Rc;
pub use generated_code::*;

use slint::VecModel;

fn app() -> App {
    let app = App::new().unwrap();

    let tab_items = Rc::new(VecModel::default());
    tab_items.extend_from_slice(&[
        TabItem {
            text: "Tab 1".into(),
            ..Default::default()
        },
        TabItem {
            text: "Tab 2".into(),
            closable: true,
            ..Default::default()
        },
        TabItem {
            text: "Tab 3".into(),
            closable: true,
            ..Default::default()
        },
        TabItem {
            text: "Tab 4".into(),
            closable: true,
            ..Default::default()
        },
    ]);

    app.global::<ListPageAdapter>().on_close_tab_item({
        let tab_items = tab_items.clone();

        move |index| {
            tab_items.remove(index as usize);
        }
    });

    app.global::<ListPageAdapter>()
        .set_tab_items(tab_items.into());

    virtual_keyboard::init(&app);

    app
}

#[cfg(not(feature = "mcu-board-support"))]
#[cfg_attr(target_arch = "wasm32", wasm_bindgen(start))]
pub fn main() {
    #[cfg(feature = "coop_client")]
    coop_client::init_config(600., 400., "widgets");

    // This provides better error messages in debug mode.
    // It's disabled in release mode so it doesn't bloat up the file size.
    #[cfg(all(debug_assertions, target_arch = "wasm32"))]
    console_error_panic_hook::set_once();

    app().run().unwrap()
}

#[cfg(feature = "mcu-board-support")]
#[mcu_board_support::entry]
fn main() -> ! {
    mcu_board_support::init();

    let app = app();

    app.global::<AppManager>().set_keyboard_enabled(true);
    app.global::<Theme>().set_settings(settings);

    app.run().unwrap();

    panic!("The MCU demo should not quit")
}

mod virtual_keyboard {
    use super::*;
    use slint::*;

    pub fn init(app: &App) {
        let weak = app.as_weak();
        app.global::<VirtualKeyboardAdapter>().on_key_pressed({
            move |key| {
                weak.unwrap()
                    .window()
                    .dispatch_event(slint::platform::WindowEvent::KeyPressed { text: key.clone() });
                weak.unwrap()
                    .window()
                    .dispatch_event(slint::platform::WindowEvent::KeyReleased { text: key });
            }
        });
    }
}
