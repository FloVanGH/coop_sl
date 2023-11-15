// SPDX-FileCopyrightText: 2023 Florian Blasius <co_sl@tutanota.com>
// SPDX-License-Identifier: GPL-3.0-only

use std::cell::Cell;
use std::cmp::Ordering;
use std::rc::Rc;

use crate::models::FileModel;
use crate::repositories::traits::BookmarksRepository;
use crate::Callback;
use crate::{models::BookmarkModel, ui::*};
use slint::*;

mod context_menu {
    pub const REMOVE_BOOKMARK: &str = "remove bookmark";
}

pub struct BookmarksController<T: BookmarksRepository> {
    bookmarks: Rc<VecModel<BookmarkModel>>,
    repository: Rc<T>,
    selected_bookmark: Cell<Option<usize>>,
    open_callback: Rc<Callback<FileModel, ()>>,
}

impl<T: BookmarksRepository + 'static> BookmarksController<T> {
    pub fn new(repository: T) -> Self {
        let bookmarks = Rc::new(VecModel::default());
        let mut bookmarks_vec = repository.load();

        bookmarks_vec.sort_by(|a, b| {
            if a.unremovable().eq(&b.unremovable()) {
                return Ordering::Equal;
            }

            if !a.unremovable() && b.unremovable() {
                Ordering::Less
            } else {
                Ordering::Greater
            }
        });
        bookmarks.set_vec(bookmarks_vec);

        Self {
            bookmarks,
            repository: Rc::new(repository),
            selected_bookmark: Cell::new(None),
            open_callback: Rc::new(Callback::default()),
        }
    }

    pub fn bookmarks(&self) -> ModelRc<BookmarkModel> {
        self.bookmarks.clone().into()
    }

    pub fn first_bookmark_path(&self) -> Option<String> {
        if self.bookmarks.row_count() > 0 {
            return self.bookmarks.row_data(0).map(|b| b.path().to_string());
        }

        None
    }

    pub fn context_menu(&self, index: usize) -> ModelRc<ListViewItem> {
        if let Some(bookmark) = self.bookmarks.row_data(index) {
            if bookmark.unremovable() {
                return Rc::new(VecModel::default()).into();
            }
        }

        VecModel::from_slice(&[ListViewItem {
            text: "Remove".into(),
            spec: context_menu::REMOVE_BOOKMARK.into(),
            ..Default::default()
        }])
    }

    pub fn select(&self, path: &str) {
        if let Some(selected_bookmark) = self.selected_bookmark.get() {
            if let Some(mut bookmark) = self.bookmarks.row_data(selected_bookmark) {
                bookmark.set_selected(false);
                self.bookmarks.set_row_data(selected_bookmark, bookmark);
            }

            self.selected_bookmark.set(None);
        }

        for r in 0..self.bookmarks.row_count() {
            if let Some(mut bookmark) = self.bookmarks.row_data(r) {
                if bookmark.path().eq(path) {
                    bookmark.set_selected(true);
                    self.bookmarks.set_row_data(r, bookmark);
                    self.selected_bookmark.set(Some(r));
                    return;
                }
            }
        }
    }

    pub fn add(&self, bookmark: BookmarkModel) {
        if self.bookmarks.iter().any(|b| b.eq(&bookmark)) {
            return;
        }

        let mut insert_index = 0;

        for i in (0..self.bookmarks.row_count()).rev() {
            if let Some(bookmark) = self.bookmarks.row_data(i) {
                if !bookmark.unremovable() {
                    break;
                }
            }

            insert_index = i;
        }

        self.bookmarks.insert(insert_index, bookmark);
        self.save();
    }

    pub fn execute_drop(&self, source: usize, target: usize) {
        if self.bookmarks.row_count() == 0 || source == target {
            return;
        }

        if let Some(bookmark) = self.bookmarks.row_data(source) {
            if bookmark.unremovable() {
                return;
            }
        }

        if let Some(bookmark) = self.bookmarks.row_data(target) {
            if bookmark.unremovable() {
                return;
            }
        }

        let source_item = self.bookmarks.remove(source);
        self.bookmarks.insert(target, source_item);
        self.save();
    }

    pub fn execute_context_menu_action(&self, index: usize, spec: &str) {
        if !spec.eq(context_menu::REMOVE_BOOKMARK) {
            return;
        }

        self.remove(index);
    }

    pub fn open(&self, item: usize) {
        if let Some(bookmark) = self.bookmarks.row_data(item) {
            self.open_callback
                .invoke(&(FileModel::new(bookmark.path())));
        }
    }

    pub fn next(&self) {
        if let Some(selected_bookmark) = self.selected_bookmark.get() {
            if selected_bookmark + 1 < self.bookmarks.row_count() {
                self.open(selected_bookmark + 1);
            } else {
                self.open(0);
            }
        }
    }

    pub fn previous(&self) {
        if let Some(selected_bookmark) = self.selected_bookmark.get() {
            if selected_bookmark > 0 {
                self.open(selected_bookmark - 1);
            }
        }
    }

    pub fn selected_item(&self) -> i32 {
        if let Some(selected_item) = self.selected_bookmark.get() {
            return selected_item as i32;
        }

        -1
    }

    pub fn update(&self) {
        let row_count = self.bookmarks.row_count();

        for i in (0..row_count).rev() {
            if let Some(bookmark) = self.bookmarks.row_data(i) {
                if !self.repository.exists(&bookmark) {
                    self.bookmarks.remove(i);
                }
            }
        }

        if self.bookmarks.row_count() < row_count {
            self.save();
        }
    }

    pub fn on_open(&self, mut callback: impl FnMut(FileModel) + 'static) {
        self.open_callback.on(move |file| {
            callback(file.clone());
        });
    }

    fn remove(&self, index: usize) {
        if let Some(bookmark) = self.bookmarks.row_data(index) {
            if bookmark.unremovable() {
                return;
            }
        }

        self.bookmarks.remove(index);
        self.save();
    }

    fn save(&self) {
        let mut bookmarks = vec![];

        for i in 0..self.bookmarks.row_count() {
            if let Some(bookmark) = self.bookmarks.row_data(i) {
                bookmarks.push(bookmark);
            }
        }

        let _ = self.repository.save(&bookmarks);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::repositories::BookmarksRepositoryMock;

    fn bookmarks() -> Vec<BookmarkModel> {
        vec![
            BookmarkModel::new(
                crate::models::BookmarkType::Dir,
                "Documents",
                "/documents",
                false,
            ),
            BookmarkModel::new(crate::models::BookmarkType::Game, "Games", "/games", false),
            BookmarkModel::new(
                crate::models::BookmarkType::Dir,
                "Pictures",
                "/pictures",
                false,
            ),
            BookmarkModel::new(crate::models::BookmarkType::Root, "Root", "/", true),
        ]
    }

    fn controller() -> BookmarksController<BookmarksRepositoryMock> {
        BookmarksController::new(BookmarksRepositoryMock::new(bookmarks()))
    }

    fn assert_model(model: &ModelRc<BookmarkModel>, bookmarks: &[BookmarkModel]) {
        for r in 0..model.row_count() {
            assert_eq!(model.row_data(r), bookmarks.get(r).cloned());
        }
    }

    #[test]
    fn test_bookmarks() {
        let controller = controller();
        let bookmarks_model = controller.bookmarks();
        let bookmarks = bookmarks();

        assert_model(&bookmarks_model, &bookmarks);
    }

    #[test]
    fn test_select() {
        let controller = controller();
        let bookmarks_model = controller.bookmarks();
        let mut bookmarks = bookmarks();

        bookmarks.get_mut(0).unwrap().set_selected(true);
        controller.select("/documents");
        assert_model(&bookmarks_model, &bookmarks);
        assert_eq!(controller.selected_item(), 0);

        bookmarks.get_mut(2).unwrap().set_selected(true);
        controller.select("/pictures");
        assert_model(&bookmarks_model, &bookmarks);
        assert_eq!(controller.selected_item(), 2);

        controller.select("/not-part-of");
        assert_model(&bookmarks_model, &bookmarks);
        assert_eq!(controller.selected_item(), -1);
    }

    #[test]
    fn test_drop() {
        let controller = controller();

        controller.execute_drop(0, 2);
        assert_eq!(controller.bookmarks.row_data(1).unwrap().name(), "Pictures");
        assert_eq!(
            controller.bookmarks.row_data(2).unwrap().name(),
            "Documents"
        );

        controller.execute_drop(2, 1);
        assert_eq!(
            controller.bookmarks.row_data(1).unwrap().name(),
            "Documents"
        );
        assert_eq!(controller.bookmarks.row_data(2).unwrap().name(), "Pictures");

        controller.execute_drop(1, 1);
        assert_eq!(
            controller.bookmarks.row_data(1).unwrap().name(),
            "Documents"
        );

        controller.execute_drop(3, 1);
        assert_eq!(
            controller.bookmarks.row_data(1).unwrap().name(),
            "Documents"
        );
        assert_eq!(controller.bookmarks.row_data(3).unwrap().name(), "Root");
    }

    #[test]
    fn test_get_first_bookmark() {
        let controller = controller();
        assert_eq!(controller.first_bookmark_path(), Some("/documents".into()));
    }

    #[test]
    fn test_execute_item_context_menu_action() {
        let controller = controller();
        let bookmarks_model = controller.bookmarks();

        controller.execute_context_menu_action(0, context_menu::REMOVE_BOOKMARK);
        assert_eq!(
            bookmarks_model.row_data(0),
            Some(BookmarkModel::new(
                crate::models::BookmarkType::Game,
                "Games",
                "/games",
                false
            ))
        );

        controller.execute_context_menu_action(2, context_menu::REMOVE_BOOKMARK);
        assert_eq!(
            bookmarks_model.row_data(2),
            Some(BookmarkModel::new(
                crate::models::BookmarkType::Root,
                "Root",
                "/",
                true
            ))
        );
    }

    #[test]
    fn test_get_item_context_menu() {
        let controller = controller();
        let context_menu_model = controller.context_menu(0);

        assert_eq!(
            &context_menu_model.row_data(0).unwrap().spec,
            context_menu::REMOVE_BOOKMARK
        );

        let context_menu_model = controller.context_menu(3);
        assert_eq!(context_menu_model.row_count(), 0);
    }

    #[test]
    fn test_open_dir() {
        let controller = controller();

        controller.on_open(|file_model| {
            assert_eq!(file_model.path(), "/documents");
        });
        controller.open(0);
    }

    #[test]
    fn test_open_next_dir() {
        let controller = controller();

        controller.on_open(|file_model| {
            assert_eq!(file_model.path(), "/games");
        });
        controller.select("/documents");
        controller.next();
    }

    #[test]
    fn test_open_previous_dir() {
        let controller = controller();

        controller.on_open(|file_model| {
            assert_eq!(file_model.path(), "/pictures");
        });
        controller.select("/");
        controller.previous();
    }
}
