// SPDX-FileCopyrightText: 2023 Florian Blasius <co_sl@tutanota.com>
// SPDX-License-Identifier: GPL-3.0-only

mod bookmarks_proxy_model;
pub use bookmarks_proxy_model::*;

mod files_proxy_model;
pub use files_proxy_model::*;

#[cfg(feature = "games")]
mod games_proxy_model;

#[cfg(feature = "games")]
pub use games_proxy_model::*;
