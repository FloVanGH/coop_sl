// SPDX-FileCopyrightText: 2023 Florian Blasius <co_sl@tutanota.com>
// SPDX-License-Identifier: GPL-3.0-only

use crate::model::BookmarkModel;
use std::io;

pub trait BookMarksRepository {
    fn add_bookmark(&self, bookmark: &BookmarkModel) -> io::Result<()>;
    fn remove_bookmark(&self, index: usize) -> io::Result<BookmarkModel>;
    fn bookmarks(&self) -> Vec<BookmarkModel>;
    fn locations(&self) -> Vec<BookmarkModel>;
}
