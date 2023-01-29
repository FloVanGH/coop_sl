// SPDX-FileCopyrightText: 2022 Florian Blasius <co_sl@tutanota.com>
// SPDX-License-Identifier: GPL-3.0-only

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

#[allow(clippy::all)]
mod generated_code {
    slint::include_modules!();
}

pub use generated_code::*;

// pub mod controller;
// pub mod mocks;
// mod view_controller;

// use view_controller::*;

#[cfg_attr(target_arch = "wasm32", wasm_bindgen(start))]
pub fn main() {
    // This provides better error messages in debug mode.
    // It's disabled in release mode so it doesn't bloat up the file size.
    #[cfg(all(debug_assertions, target_arch = "wasm32"))]
    console_error_panic_hook::set_once();

    #[cfg(feature = "slint_coop")]
    slint_coop::init_config(600., 400., "coop_chat");

    let app = App::new();

    // let _side_bar_view_controller =
    //     SideBarViewController::new(&app, Box::new(mocks::MockSideBarController::new()));

    app.run();
}
