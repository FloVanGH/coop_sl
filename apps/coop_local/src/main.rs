// SPDX-FileCopyrightText: 2023 Florian Blasius <co_sl@tutanota.com>
// SPDX-License-Identifier: GPL-3.0-only
use slint::*;

use coop_local::{controller, repository, ui};

pub fn main() -> Result<(), slint::PlatformError> {
    let main_window = ui::MainWindow::new()?;

    let dialog_view_controller = controller::DialogViewController::new(&main_window)
        .map_err(|e| slint::PlatformError::Other(e.to_string()))?;

    #[cfg(feature = "mock")]
    let items_view_controller = controller::FilesController::new(
        &main_window,
        repository::FileRepositoryMock::new(),
        dialog_view_controller,
    );
    #[cfg(not(feature = "mock"))]
    let items_view_controller = controller::FilesController::new(
        &main_window,
        repository::FileRepository::new(),
        dialog_view_controller,
    );

    let start_dir = if let Some(home) = dirs::home_dir() {
        home.as_path().to_str().unwrap_or_default().to_string()
    } else {
        "/".to_string()
    };

    items_view_controller
        .map_err(|e| slint::PlatformError::Other(e.to_string()))?
        .spawn_message(controller::FilesMessage::Load { path: start_dir });

    main_window.run()
}
