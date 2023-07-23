// SPDX-FileCopyrightText: 2023 Florian Blasius <co_sl@tutanota.com>
// SPDX-License-Identifier: GPL-3.0-only

use slint::*;

#[allow(clippy::all)]
pub mod ui {
    slint::include_modules!();
}

pub fn main() -> Result<(), slint::PlatformError> {
    let main_window = ui::MainWindow::new()?;
    main_window.run()
}
