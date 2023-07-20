// SPDX-FileCopyrightText: 2022 Florian Blasius <co_sl@tutanota.com>
// SPDX-License-Identifier: MIT

#[allow(clippy::all)]
mod generated_code {
    slint::include_modules!();
}

pub use generated_code::*;

pub mod view_controller;

use view_controller::DisplayViewController;

fn main() {
    let ui = Desktop::new().unwrap();

    let wm_controller = DisplayViewController::new(&ui);

    ui.run().unwrap();

    wm_controller.join().unwrap();
}
