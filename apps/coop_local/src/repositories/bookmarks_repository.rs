// SPDX-FileCopyrightText: 2023 Florian Blasius <co_sl@tutanota.com>
// SPDX-License-Identifier: GPL-3.0-only

use super::traits;
use crate::{
    models::{BookmarkModel, BookmarkType},
    services::SettingsService,
};
use serde::{Deserialize, Serialize};
use std::{io, path::Path};

const BOOKMARKS: &str = "bookmarks";

#[derive(Clone, Debug, Default, Serialize, Deserialize, PartialEq, Eq)]
pub struct BookmarksVec {
    items: Vec<BookmarkModel>,
}

pub struct BookmarksRepository {
    settings_service: SettingsService,
}

impl BookmarksRepository {
    pub fn new(settings_service: SettingsService) -> Self {
        Self { settings_service }
    }
}

impl traits::BookmarksRepository for BookmarksRepository {
    fn load(&self) -> Vec<BookmarkModel> {
        let mut bookmarks =
            if let Ok(bookmarks) = self.settings_service.load::<BookmarksVec>(BOOKMARKS) {
                bookmarks
            } else {
                BookmarksVec::default()
            };

        if bookmarks.items.is_empty() {
            bookmarks.items.push(root());

            if let Some(volumes) = volumes() {
                bookmarks.items.push(volumes);
            }
        }

        bookmarks.items
    }

    fn save(&self, bookmarks: &[BookmarkModel]) -> io::Result<()> {
        let bookmarks_vec = BookmarksVec {
            items: bookmarks.to_vec(),
        };

        self.settings_service.save(BOOKMARKS, &bookmarks_vec)
    }

    fn exists(&self, bookmark: &BookmarkModel) -> bool {
        Path::new(bookmark.path()).exists()
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
