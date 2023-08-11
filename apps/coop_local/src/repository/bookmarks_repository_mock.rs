// SPDX-FileCopyrightText: 2023 Florian Blasius <co_sl@tutanota.com>
// SPDX-License-Identifier: GPL-3.0-only

use super::traits;
use crate::model::{BookmarkModel, BookmarkType};
use std::io;

#[derive(Clone)]
pub struct BookMarksRepositoryMock {}

impl BookMarksRepositoryMock {
    pub fn new() -> Self {
        Self {}
    }
}

impl traits::BookMarksRepository for BookMarksRepositoryMock {
    fn add_bookmark(&self, bookmark: &BookmarkModel) -> io::Result<()> {
        println!("Added bookmark {:?}", bookmark);
        Ok(())
    }

    fn remove_bookmark(&self, _index: usize) -> io::Result<BookmarkModel> {
        Ok(BookmarkModel::default())
    }

    fn bookmarks(&self) -> Vec<BookmarkModel> {
        vec![
            BookmarkModel::new(BookmarkType::Dir, "Documents", "/docs"),
            BookmarkModel::new(BookmarkType::Dir, "Sources", "/src"),
        ]
    }

    fn locations(&self) -> Vec<crate::model::BookmarkModel> {
        vec![BookmarkModel::new(BookmarkType::Root, "root", "/")]
    }
}
