// SPDX-FileCopyrightText: 2022 Florian Blasius <co_sl@tutanota.com>
// SPDX-License-Identifier: MIT

#![allow(clippy::redundant_clone)]
#![allow(clippy::cmp_owned)]
#![cfg_attr(any(feature = "mcu-board-support", feature = "slint_psp"), no_std)]
#![cfg_attr(any(feature = "mcu-board-support", feature = "slint_psp"), no_main)]

// #[cfg(feature = "mcu-board-support")]
extern crate alloc;

use alloc::string::ToString;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

slint::include_modules!();

fn app() -> App {
    let app = App::new();

    // this need to implemented to handle the backspace on the on screen keyboard.
    app.global::<KeyboardAdapter>().on_backspace(|text| {
        let mut text = text.to_string();
        text.pop();
        text.into()
    });

    app
}

#[cfg(not(feature = "mcu-board-support"))]
#[cfg_attr(target_arch = "wasm32", wasm_bindgen(start))]
pub fn main() {
    #[cfg(feature = "slint_orbclient")]
    slint_orbclient::init();

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
    let mut settings = app.global::<co>().get_settings();
    settings.minimize = true;
    app.global::<co>().set_settings(settings);

    app.run();

    panic!("The MCU demo should not quit")
}

#[cfg(feature = "slint_psp")]
psp::module!("module_widgets", 1, 1);

#[cfg(feature = "slint_psp")]
fn psp_main() {
    psp::enable_home_button();
    psp::dprint!("Hello PSP from rust!");
}
