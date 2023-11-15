// SPDX-FileCopyrightText: 2023 Florian Blasius <co_sl@tutanota.com>
// SPDX-License-Identifier: GPL-3.0-only

use crate::models::BookmarkModel;
use std::io;

pub trait BookmarksRepository {
    fn load(&self) -> Vec<BookmarkModel>;
    fn save(&self, bookmarks: &[BookmarkModel]) -> io::Result<()>;
    fn exists(&self, bookmark: &BookmarkModel) -> bool;
}
