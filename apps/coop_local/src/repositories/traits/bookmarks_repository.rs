// SPDX-FileCopyrightText: 2023 Florian Blasius <co_sl@tutanota.com>
// SPDX-License-Identifier: GPL-3.0-only

use crate::models::BookmarkModel;
use std::io;

pub trait BookmarksRepository {
    fn add_bookmark(&self, bookmark: BookmarkModel) -> io::Result<()>;
    fn remove_bookmark(&self, index: usize) -> io::Result<BookmarkModel>;
    fn reorder(&self, source: usize, target: usize) -> io::Result<bool>;
    fn bookmarks(&self) -> Vec<BookmarkModel>;
}
