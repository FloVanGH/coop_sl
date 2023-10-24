// SPDX-FileCopyrightText: 2023 Florian Blasius <co_sl@tutanota.com>
// SPDX-License-Identifier: GPL-3.0-only

use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum BookmarkType {
    Root,
    Dir,
    Game,
    Volume,
}

impl Default for BookmarkType {
    fn default() -> Self {
        Self::Dir
    }
}

#[derive(Clone, Default, Debug, Serialize, Deserialize, Eq)]
pub struct BookmarkModel {
    bookmark_type: BookmarkType,
    name: String,
    path: String,
    unremovable: bool,

    #[serde(skip)]
    selected: bool,
}

impl PartialEq for BookmarkModel {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name && self.path == other.path
    }
}

impl BookmarkModel {
    pub fn new(
        bookmark_type: BookmarkType,
        name: impl Into<String>,
        path: impl Into<String>,
        unremovable: bool,
    ) -> Self {
        Self {
            bookmark_type,
            name: name.into(),
            path: path.into(),
            unremovable,
            selected: false,
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

    pub fn set_selected(&mut self, selected: bool) {
        self.selected = selected;
    }

    pub fn selected(&self) -> bool {
        self.selected
    }

    pub fn unremovable(&self) -> bool {
        self.unremovable
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bookmark_model() {
        let mut bookmark_model =
            BookmarkModel::new(BookmarkType::Dir, "Bookmark", "Path/to/bookmark", false);

        assert_eq!(bookmark_model.name(), "Bookmark");
        assert_eq!(bookmark_model.path(), "Path/to/bookmark");
        assert_eq!(bookmark_model.bookmark_type(), BookmarkType::Dir);
        assert!(!bookmark_model.selected());

        bookmark_model.set_selected(true);
        assert!(bookmark_model.selected());
    }
}
