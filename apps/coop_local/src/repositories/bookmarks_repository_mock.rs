// SPDX-FileCopyrightText: 2023 Florian Blasius <co_sl@tutanota.com>
// SPDX-License-Identifier: GPL-3.0-only

use crate::models::BookmarkModel;
use std::cell::RefCell;

pub struct BookmarksRepositoryMock {
    bookmarks: RefCell<Vec<BookmarkModel>>,
}

impl BookmarksRepositoryMock {
    pub fn new(bookmarks: Vec<BookmarkModel>) -> Self {
        Self {
            bookmarks: RefCell::new(bookmarks),
        }
    }
}

impl crate::repositories::traits::BookmarksRepository for BookmarksRepositoryMock {
    fn load(&self) -> Vec<BookmarkModel> {
        self.bookmarks.borrow().clone()
    }

    fn save(&self, _bookmarks: &[BookmarkModel]) -> std::io::Result<()> {
        Ok(())
    }

    fn exists(&self, _bookmark: &BookmarkModel) -> bool {
        true
    }
}
