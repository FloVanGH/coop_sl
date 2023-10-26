// SPDX-FileCopyrightText: 2023 Florian Blasius <co_sl@tutanota.com>
// SPDX-License-Identifier: GPL-3.0-only

mod bookmarks_repository_mock;
pub use bookmarks_repository_mock::*;

mod bookmarks_repository;
pub use bookmarks_repository::*;

mod file_repository;
pub use file_repository::*;

#[cfg(feature = "games")]
mod games_repository;
#[cfg(feature = "games")]
pub use games_repository::*;

mod image_repository;
pub use image_repository::*;

mod text_repository;
pub use text_repository::*;

pub mod traits;
