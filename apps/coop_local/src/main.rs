// SPDX-FileCopyrightText: 2023 Florian Blasius <co_sl@tutanota.com>
// SPDX-License-Identifier: GPL-3.0-only
use slint::*;

use coop_local::{controller, repository, service::SettingsService, ui};

pub fn main() -> Result<(), slint::PlatformError> {
    let main_window = ui::MainWindow::new()?;

    let dialog_view_controller = controller::DialogViewController::new(&main_window)
        .map_err(|e| slint::PlatformError::Other(e.to_string()))?;

    #[cfg(feature = "mock")]
    let files_repository = repository::FileRepositoryMock::new();

    #[cfg(not(feature = "mock"))]
    let files_repository = repository::FileRepository::new();

    let files_controller =
        controller::FilesController::new(&main_window, files_repository, dialog_view_controller)
            .map_err(|e| slint::PlatformError::Other(e.to_string()))?;

    let start_dir = if let Some(home) = dirs::home_dir() {
        home.as_path().to_str().unwrap_or_default().to_string()
    } else {
        "/".to_string()
    };

    files_controller.spawn_message(controller::FilesMessage::Load { path: start_dir });

    let update_timer = Timer::default();
    update_timer.start(
        TimerMode::Repeated,
        std::time::Duration::from_millis(500),
        move || {
            files_controller.spawn_message(controller::FilesMessage::CheckForModifications);
        },
    );

    #[cfg(feature = "mock")]
    let bookmarks_repository = repository::BookMarksRepositoryMock::new();

    #[cfg(not(feature = "mock"))]
    let bookmarks_repository = repository::BookMarksRepository::new(
        SettingsService::new().map_err(|e| slint::PlatformError::Other(e.to_string()))?,
    );

    controller::SideBarController::new(&main_window, bookmarks_repository)
        .map_err(|e| slint::PlatformError::Other(e.to_string()))?;

    main_window.run()
}
