// SPDX-FileCopyrightText: 2023 Florian Blasius <co_sl@tutanota.com>
// SPDX-License-Identifier: GPL-3.0-only

mod bookmarks_repository;
pub use bookmarks_repository::*;

mod bookmarks_repository_mock;
pub use bookmarks_repository_mock::*;

mod file_repository;
pub use file_repository::*;

mod file_repository_mock;
pub use file_repository_mock::*;

pub mod traits;
