// SPDX-FileCopyrightText: 2023 Florian Blasius <co_sl@tutanota.com>
// SPDX-License-Identifier: GPL-3.0-only

mod bookmarks_repository;
pub use bookmarks_repository::*;

mod file_repository;
pub use file_repository::*;

#[cfg(feature = "games")]
mod games_repository;
#[cfg(feature = "games")]
pub use games_repository::*;
