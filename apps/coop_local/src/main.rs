// SPDX-FileCopyrightText: 2023 Florian Blasius <co_sl@tutanota.com>
// SPDX-License-Identifier: GPL-3.0-only

use slint::*;

mod controller;
mod model;
mod repository;
mod ui;

pub fn main() -> Result<(), slint::PlatformError> {
    let main_window = ui::MainWindow::new()?;

    #[cfg(feature = "mock")]
    let items_view_controller =
        controller::ItemsViewController::new(&main_window, repository::ItemsRepositoryMock::new());
    #[cfg(not(feature = "mock"))]
    let items_view_controller =
        controller::ItemsViewController::new(&main_window, repository::FileItemsRepository::new());
    items_view_controller
        .spawn_message(controller::ItemsViewMessage::DisplayItems { root: "/".into() });

    main_window.run()
}
