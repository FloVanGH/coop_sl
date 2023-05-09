// SPDX-FileCopyrightText: 2022 Florian Blasius <co_sl@tutanota.com>
// SPDX-License-Identifier: MIT

#![cfg_attr(feature = "mcu-board-support", no_std)]
#![cfg_attr(feature = "mcu-board-support", no_main)]

// #[cfg(feature = "mcu-board-support")]
extern crate alloc;

use alloc::string::ToString;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

#[allow(clippy::all)]
mod generated_code {
    slint::include_modules!();
}

pub use generated_code::*;

fn app() -> App {
    let app = App::new().unwrap();

    // this need to implemented to handle the backspace on the on screen keyboard.
    app.global::<KeyboardAdapter>().on_backspace(|text| {
        let mut text = text.to_string();
        text.pop();
        text.into()
    });

    #[cfg(target_os = "redox")]
    app.global::<Theme>().set_embedded_helper(true);

    #[cfg(feature = "coop_client")]
    app.global::<Theme>().set_skip_animations(true);

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
    let mut settings = app.global::<Theme>().get_settings();
    settings.minimize = true;
    app.global::<Theme>().set_settings(settings);

    app.run().unwrap();

    panic!("The MCU demo should not quit")
}
