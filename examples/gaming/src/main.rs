// SPDX-FileCopyrightText: 2022 Florian Blasius <flovanpt@posteo.de>
// SPDX-License-Identifier: MIT

#![allow(clippy::redundant_clone)]
#![allow(clippy::cmp_owned)]
#![cfg_attr(not(feature = "std"), no_std)]
#![cfg_attr(not(feature = "std"), no_main)]

#[cfg(not(feature = "std"))]
extern crate alloc;

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

slint::include_modules!();

#[cfg(feature = "std")]
#[cfg_attr(target_arch = "wasm32", wasm_bindgen(start))]
pub fn main() {
    // This provides better error messages in debug mode.
    // It's disabled in release mode so it doesn't bloat up the file size.
    #[cfg(all(debug_assertions, target_arch = "wasm32"))]
    console_error_panic_hook::set_once();

    Game::new().run()
}

#[cfg(feature = "mcu-board-support")]
#[mcu_board_support::entry]
fn main() -> ! {
    mcu_board_support::init();
    Game::new().run();

    panic!("Should not quit")
}
