// SPDX-FileCopyrightText: 2022 Florian Blasius <co_sl@tutanota.com>
// SPDX-License-Identifier: MIT

#![no_std]
#![cfg_attr(not(feature = "simulator"), no_main)]

extern crate alloc;

slint::include_modules!();

fn create_slint_app() -> App {
    App::new()
}

#[cfg(feature = "simulator")]
fn main() {
    create_slint_app().run();
}

#[cfg(not(feature = "simulator"))]
psp::module!("sample_test", 1, 1);

#[cfg(not(feature = "simulator"))]
fn psp_main() {
    psp::enable_home_button();

    slint_psp::init();

    create_slint_app().run();
}
