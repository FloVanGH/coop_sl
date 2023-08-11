// SPDX-FileCopyrightText: 2023 Florian Blasius <co_sl@tutanota.com>
// SPDX-License-Identifier: GPL-3.0-only

use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum BookmarkType {
    Root,
    Dir,
}

impl Default for BookmarkType {
    fn default() -> Self {
        Self::Dir
    }
}

#[derive(Clone, Default, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct BookmarkModel {
    bookmark_type: BookmarkType,
    name: String,
    path: String,
}

impl BookmarkModel {
    pub fn new(
        bookmark_type: BookmarkType,
        name: impl Into<String>,
        path: impl Into<String>,
    ) -> Self {
        Self {
            bookmark_type,
            name: name.into(),
            path: path.into(),
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn path(&self) -> &str {
        &self.path
    }

    pub fn bookmark_type(&self) -> BookmarkType {
        self.bookmark_type
    }
}
