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
    fn add_bookmark(&self, bookmark: BookmarkModel) -> std::io::Result<()> {
        self.bookmarks.borrow_mut().push(bookmark.clone());

        Ok(())
    }

    fn remove_bookmark(&self, index: usize) -> std::io::Result<BookmarkModel> {
        Ok(self.bookmarks.borrow_mut().remove(index))
    }

    fn reorder(&self, source: usize, target: usize) -> std::io::Result<bool> {
        let len = self.bookmarks.borrow().len();

        if source >= len || target >= len {
            return Ok(false);
        }

        let bookmark = self.bookmarks.borrow_mut().remove(source);
        self.bookmarks.borrow_mut().insert(target, bookmark);

        Ok(true)
    }

    fn bookmarks(&self) -> Vec<BookmarkModel> {
        self.bookmarks.borrow().clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::repositories::traits::BookmarksRepository;

    #[test]
    fn test_add_bookmark() {
        let bookmarks = vec![BookmarkModel::new(
            crate::models::BookmarkType::Dir,
            "Bookmark 1",
            "",
            false,
        )];

        let repository = BookmarksRepositoryMock::new(bookmarks.clone());
        assert_eq!(repository.bookmarks(), bookmarks);

        let new_bookmark =
            BookmarkModel::new(crate::models::BookmarkType::Dir, "Bookmark 2", "", false);

        repository.add_bookmark(new_bookmark.clone()).unwrap();

        assert_eq!(repository.bookmarks().last(), Some(&new_bookmark))
    }

    #[test]
    fn test_remove_bookmark() {
        let bookmarks = vec![
            BookmarkModel::new(crate::models::BookmarkType::Dir, "Bookmark 1", "", false),
            BookmarkModel::new(crate::models::BookmarkType::Dir, "Bookmark 2", "", false),
        ];

        let repository = BookmarksRepositoryMock::new(bookmarks.clone());
        assert_eq!(
            repository.remove_bookmark(1).unwrap(),
            BookmarkModel::new(crate::models::BookmarkType::Dir, "Bookmark 2", "", false)
        );
        assert_eq!(
            repository.bookmarks().last(),
            Some(&BookmarkModel::new(
                crate::models::BookmarkType::Dir,
                "Bookmark 1",
                "",
                false
            ))
        );
    }

    #[test]
    fn test_reorder() {
        let bookmarks = vec![
            BookmarkModel::new(crate::models::BookmarkType::Dir, "Bookmark 1", "", false),
            BookmarkModel::new(crate::models::BookmarkType::Dir, "Bookmark 2", "", false),
            BookmarkModel::new(crate::models::BookmarkType::Dir, "Bookmark 3", "", false),
            BookmarkModel::new(crate::models::BookmarkType::Dir, "Bookmark 4", "", false),
        ];
        let repository = BookmarksRepositoryMock::new(bookmarks.clone());

        repository.reorder(0, 3).unwrap();
        let bookmarks = vec![
            BookmarkModel::new(crate::models::BookmarkType::Dir, "Bookmark 2", "", false),
            BookmarkModel::new(crate::models::BookmarkType::Dir, "Bookmark 3", "", false),
            BookmarkModel::new(crate::models::BookmarkType::Dir, "Bookmark 4", "", false),
            BookmarkModel::new(crate::models::BookmarkType::Dir, "Bookmark 1", "", false),
        ];
        assert_eq!(repository.bookmarks(), bookmarks);

        repository.reorder(2, 1).unwrap();
        let bookmarks = vec![
            BookmarkModel::new(crate::models::BookmarkType::Dir, "Bookmark 2", "", false),
            BookmarkModel::new(crate::models::BookmarkType::Dir, "Bookmark 4", "", false),
            BookmarkModel::new(crate::models::BookmarkType::Dir, "Bookmark 3", "", false),
            BookmarkModel::new(crate::models::BookmarkType::Dir, "Bookmark 1", "", false),
        ];
        assert_eq!(repository.bookmarks(), bookmarks);

        repository.reorder(0, 0).unwrap();
        assert_eq!(repository.bookmarks(), bookmarks);
    }
}
