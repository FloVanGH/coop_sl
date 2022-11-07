#![allow(clippy::redundant_clone)]
#![allow(clippy::cmp_owned)]
#![cfg_attr(feature = "mcu-board-support", no_std)]
#![cfg_attr(all(feature = "mcu-board-support", not(simulator)), no_main)]

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
    #[cfg(feature = "slint_redox")]
    slint_redox::init();

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

    app.global::<settings>().set_keyboard_enabled(true);
    let mut settings = app.global::<co>().get_settings();
    settings.minimize = true;
    app.global::<co>().set_settings(settings);

    app.run();

    panic!("The MCU demo should not quit")
}
