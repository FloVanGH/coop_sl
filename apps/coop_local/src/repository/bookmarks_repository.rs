// SPDX-FileCopyrightText: 2023 Florian Blasius <co_sl@tutanota.com>
// SPDX-License-Identifier: GPL-3.0-only
use super::traits;
use crate::{
    model::{BookmarkModel, BookmarkType},
    service::SettingsService,
};
use serde::{Deserialize, Serialize};
use std::{
    io,
    sync::{Arc, Mutex},
};

const BOOKMARKS: &str = "bookmarks";

#[derive(Clone, Debug, Default, Serialize, Deserialize, PartialEq, Eq)]
pub struct BookmarksVec {
    bookmarks: Vec<BookmarkModel>,
}

#[derive(Clone)]
pub struct BookMarksRepository {
    settings_service: SettingsService,
    bookmarks: Arc<Mutex<BookmarksVec>>,
}

impl BookMarksRepository {
    pub fn new(settings_service: SettingsService) -> Self {
        let bookmarks = if let Ok(bookmarks) = settings_service.load::<BookmarksVec>(BOOKMARKS) {
            bookmarks
        } else {
            BookmarksVec::default()
        };

        Self {
            settings_service,
            bookmarks: Arc::new(Mutex::new(bookmarks)),
        }
    }
}

impl traits::BookMarksRepository for BookMarksRepository {
    fn add_bookmark(&self, bookmark: &BookmarkModel) -> io::Result<()> {
        let mut bookmark_vec = self
            .bookmarks
            .lock()
            .map_err(|e| io::Error::new(io::ErrorKind::Other, e.to_string()))?;
        bookmark_vec.bookmarks.push(bookmark.clone());
        self.settings_service.save(BOOKMARKS, &*bookmark_vec)
    }

    fn remove_bookmark(&self, index: usize) -> io::Result<BookmarkModel> {
        let mut bookmark_vec = self
            .bookmarks
            .lock()
            .map_err(|e| io::Error::new(io::ErrorKind::Other, e.to_string()))?;
        let removed_bookmark = bookmark_vec.bookmarks.remove(index);

        self.settings_service.save(BOOKMARKS, &*bookmark_vec)?;

        Ok(removed_bookmark)
    }

    fn bookmarks(&self) -> Vec<BookmarkModel> {
        if let Ok(bookmarks) = self.bookmarks.lock() {
            return bookmarks.bookmarks.clone();
        }

        vec![]
    }

    fn locations(&self) -> Vec<crate::model::BookmarkModel> {
        let root = if cfg!(target_os = "windows") {
            BookmarkModel::new(BookmarkType::Root, "C", "C:\\")
        } else {
            BookmarkModel::new(BookmarkType::Root, "root", "/")
        };

        vec![root]
    }
}
