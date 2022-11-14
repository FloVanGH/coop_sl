// SPDX-FileCopyrightText: 2022 Florian Blasius <co_sl@tutanota.com>
// SPDX-License-Identifier: MIT

#![allow(clippy::redundant_clone)]
#![allow(clippy::cmp_owned)]
#![cfg_attr(feature = "mcu-board-support", no_std)]
#![cfg_attr(feature = "mcu-board-support", no_main)]

slint::include_modules!();

fn os() -> Os {
    Os::new()
}

#[cfg(not(feature = "mcu-board-support"))]
#[cfg_attr(target_arch = "wasm32", wasm_bindgen(start))]
fn main() {
    // This provides better error messages in debug mode.
    // It's disabled in release mode so it doesn't bloat up the file size.
    #[cfg(all(debug_assertions, target_arch = "wasm32"))]
    console_error_panic_hook::set_once();

    os().run();
}

#[cfg(feature = "mcu-board-support")]
#[mcu_board_support::entry]
fn main() -> ! {
    mcu_board_support::init();

    os().run();

    panic!("The MCU demo should not quit")
}
