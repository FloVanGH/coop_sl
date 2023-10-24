// SPDX-FileCopyrightText: 2023 Florian Blasius <co_sl@tutanota.com>
// SPDX-License-Identifier: GPL-3.0-only

use crate::{
    models::{BookmarkModel, BookmarkType},
    services::SettingsService,
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
pub struct BookmarksRepository {
    settings_service: SettingsService,
    bookmarks: Arc<Mutex<BookmarksVec>>,
}

impl BookmarksRepository {
    pub fn new(settings_service: SettingsService) -> Self {
        let mut bookmarks = if let Ok(bookmarks) = settings_service.load::<BookmarksVec>(BOOKMARKS)
        {
            bookmarks
        } else {
            BookmarksVec::default()
        };

        if bookmarks.bookmarks.is_empty() {
            bookmarks.bookmarks.push(root());

            if let Some(volumes) = volumes() {
                bookmarks.bookmarks.push(volumes);
            }
        }

        Self {
            settings_service,
            bookmarks: Arc::new(Mutex::new(bookmarks)),
        }
    }

    pub fn add_bookmark(&self, bookmark: &BookmarkModel) -> io::Result<()> {
        let mut bookmark_vec = self
            .bookmarks
            .lock()
            .map_err(|e| io::Error::new(io::ErrorKind::Other, e.to_string()))?;
        bookmark_vec.bookmarks.push(bookmark.clone());
        self.settings_service.save(BOOKMARKS, &*bookmark_vec)
    }

    pub fn reorder(&self, source: usize, target: usize) -> io::Result<bool> {
        if source == target {
            return Ok(false);
        }

        let mut bookmark_vec = self
            .bookmarks
            .lock()
            .map_err(|e| io::Error::new(io::ErrorKind::Other, e.to_string()))?;

        if source >= bookmark_vec.bookmarks.len() || target >= bookmark_vec.bookmarks.len() {
            return Ok(false);
        }

        let bookmark = bookmark_vec.bookmarks.remove(source);
        bookmark_vec.bookmarks.insert(target, bookmark);

        self.settings_service.save(BOOKMARKS, &*bookmark_vec)?;

        Ok(true)
    }

    pub fn remove_bookmark(&self, index: usize) -> io::Result<BookmarkModel> {
        let mut bookmark_vec = self
            .bookmarks
            .lock()
            .map_err(|e| io::Error::new(io::ErrorKind::Other, e.to_string()))?;
        let removed_bookmark = bookmark_vec.bookmarks.remove(index);

        self.settings_service.save(BOOKMARKS, &*bookmark_vec)?;

        Ok(removed_bookmark)
    }

    pub fn bookmarks(&self) -> Vec<BookmarkModel> {
        if let Ok(bookmarks) = self.bookmarks.lock() {
            return bookmarks.bookmarks.clone();
        }

        vec![]
    }
}

pub fn root() -> BookmarkModel {
    if cfg!(target_os = "windows") {
        BookmarkModel::new(BookmarkType::Root, "C", "C:\\", true)
    } else {
        BookmarkModel::new(BookmarkType::Root, "root", "/", true)
    }
}

pub fn volumes() -> Option<BookmarkModel> {
    if cfg!(target_os = "macos") {
        return Some(BookmarkModel::new(
            BookmarkType::Volume,
            "Volumes",
            "/Volumes",
            true,
        ));
    }

    None
}
