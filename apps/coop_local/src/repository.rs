// SPDX-FileCopyrightText: 2023 Florian Blasius <co_sl@tutanota.com>
// SPDX-License-Identifier: GPL-3.0-only

#[cfg(not(feature = "mock"))]
mod file_items_repository;
#[cfg(not(feature = "mock"))]
pub use file_items_repository::*;

#[cfg(feature = "mock")]
mod items_repository_mock;
#[cfg(feature = "mock")]
pub use items_repository_mock::*;

mod items_repository;
pub use items_repository::*;
