// SPDX-FileCopyrightText: 2022 Florian Blasius <flovanpt@posteo.de>
// SPDX-License-Identifier: GPL-3.0-only

#![allow(clippy::redundant_clone)]
#![allow(clippy::cmp_owned)]

[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

slint::include_modules!();

#[cfg_attr(target_arch = "wasm32", wasm_bindgen(start))]
fn main() {
    // This provides better error messages in debug mode.
    // It's disabled in release mode so it doesn't bloat up the file size.
    #[cfg(all(debug_assertions, target_arch = "wasm32"))]
    console_error_panic_hook::set_once();

    let ui = AppWindow::new();

    let ui_handle = ui.as_weak();
    ui.on_request_increase_value(move || {
        let ui = ui_handle.unwrap();
        ui.set_counter(ui.get_counter() + 1);
    });

    ui.run();
}
