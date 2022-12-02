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
    let app = App::new();

    // this need to implemented to handle the backspace on the on screen keyboard.
    app.global::<KeyboardAdapter>().on_backspace(|text| {
        let mut text = text.to_string();
        text.pop();
        text.into()
    });

    #[cfg(feature = "slint_orbclient")]
    app.global::<coop>().set_embedded_helper(true);

    #[cfg(feature = "slint_coop")]
    app.global::<coop>().set_skip_animations(true);

    app
}

#[cfg(not(feature = "mcu-board-support"))]
#[cfg_attr(target_arch = "wasm32", wasm_bindgen(start))]
pub fn main() {
    #[cfg(feature = "slint_orbclient")]
    slint_orbclient::init_config(
        slint_orbclient::Config::default()
            .width(1000)
            .height(600)
            .resizable(true)
            .events_async(true)
            .title("Widgets gallery"),
    );

    #[cfg(feature = "slint_coop")]
    slint_coop::init_config(600., 400., "widgets");

    // This provides better error messages in debug mode.
    // It's disabled in release mode so it doesn't bloat up the file size.
    #[cfg(all(debug_assertions, target_arch = "wasm32"))]
    console_error_panic_hook::set_once();

    app().run()
}

#[cfg(feature = "mcu-board-support")]
#[mcu_board_support::entry]
fn main() -> ! {
    mcu_board_support::init();

    let app = app();

    app.global::<AppManager>().set_keyboard_enabled(true);
    let mut settings = app.global::<coop>().get_settings();
    settings.minimize = true;
    app.global::<coop>().set_settings(settings);

    app.run();

    panic!("The MCU demo should not quit")
}
