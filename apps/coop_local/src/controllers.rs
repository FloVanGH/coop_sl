// SPDX-FileCopyrightText: 2023 Florian Blasius <co_sl@tutanota.com>
// SPDX-License-Identifier: GPL-3.0-only

mod bookmarks_controller;
pub use bookmarks_controller::*;

mod dialog_controller;
pub use dialog_controller::*;

mod files_controller;
pub use files_controller::*;

#[cfg(feature = "games")]
mod games_controller;

#[cfg(feature = "games")]
pub use games_controller::*;

mod image_controller;
pub use image_controller::*;

mod text_controller;
pub use text_controller::*;
