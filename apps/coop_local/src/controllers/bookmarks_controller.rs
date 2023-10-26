// SPDX-FileCopyrightText: 2023 Florian Blasius <co_sl@tutanota.com>
// SPDX-License-Identifier: GPL-3.0-only

use std::cell::Cell;
use std::cell::RefCell;
use std::path::Path;
use std::rc::Rc;

use crate::models::FileModel;
use crate::repositories::traits::BookmarksRepository;
use crate::{models::BookmarkModel, ui::*};
use slint::*;

mod context_menu {
    pub const REMOVE_BOOKMARK: &str = "remove bookmark";
}

type FileCallback = Rc<RefCell<Box<dyn FnMut(FileModel) + 'static>>>;

pub struct BookmarksController<T: BookmarksRepository> {
    bookmarks: Rc<VecModel<BookmarkModel>>,
    repository: Rc<T>,
    selected_bookmark: Cell<Option<usize>>,
    open_internal_callback: FileCallback,
}

impl<T: BookmarksRepository + 'static> BookmarksController<T> {
    pub fn new(repository: T) -> Self {
        let bookmarks = Rc::new(VecModel::default());

        let controller = Self {
            bookmarks,
            repository: Rc::new(repository),
            selected_bookmark: Cell::new(None),
            open_internal_callback: Rc::new(RefCell::new(Box::new(|_f: FileModel| {}))),
        };

        controller.load_bookmarks();

        controller
    }

    pub fn bookmarks(&self) -> ModelRc<BookmarkModel> {
        self.bookmarks.clone().into()
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

    pub fn add_bookmark(&self, bookmark: BookmarkModel) {
        if self.bookmarks.iter().any(|b| b.eq(&bookmark)) {
            return;
        }

        let repository = self.repository.clone();
        let bookmarks = self.bookmarks.clone();

        let _ = slint::spawn_local(async move {
            if repository.add_bookmark(bookmark.clone()).is_ok() {
                bookmarks.push(bookmark);
            }
        });
    }

    pub fn reorder(&self, source: usize, target: usize) {
        if let Ok(reorder) = self.repository.reorder(source, target) {
            if reorder {
                self.bookmarks.set_vec(self.repository.bookmarks());
            }
        }
    }

    pub fn on_open_internal(&self, callback: impl FnMut(FileModel) + 'static) {
        *self.open_internal_callback.borrow_mut() = Box::new(callback);
    }

    pub fn first_bookmark_path(&self) -> Option<String> {
        if self.bookmarks.row_count() > 0 {
            return self.bookmarks.row_data(0).map(|b| b.path().to_string());
        }

        None
    }

    pub fn execute_item_context_menu_action(&self, index: usize, spec: &str) {
        if !spec.eq(context_menu::REMOVE_BOOKMARK) {
            return;
        }

        if let Some(bookmark) = self.bookmarks.row_data(index) {
            if bookmark.unremovable() {
                return;
            }
        }

        if self.repository.remove_bookmark(index).is_ok() {
            self.bookmarks.remove(index);
        }
    }

    pub fn get_item_context_menu(&self, index: usize) -> ModelRc<ListViewItem> {
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

    pub fn open_dir(&self, item: usize) {
        if let Some(bookmark) = self.bookmarks.row_data(item) {
            let mut r: std::cell::RefMut<'_, Box<dyn FnMut(FileModel)>> =
                self.open_internal_callback.try_borrow_mut().unwrap();
            r(FileModel::new(bookmark.path()));
        }
    }

    pub fn open_next_dir(&self) {
        if let Some(selected_bookmark) = self.selected_bookmark.get() {
            if selected_bookmark + 1 < self.bookmarks.row_count() {
                self.open_dir(selected_bookmark + 1);
            } else {
                self.open_dir(0);
            }
        }
    }

    pub fn open_previous_dir(&self) {
        if let Some(selected_bookmark) = self.selected_bookmark.get() {
            if selected_bookmark > 0 {
                self.open_dir(selected_bookmark - 1);
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
        let bookmarks = self.bookmarks.clone();
        let repository = self.repository.clone();

        let _ = slint::spawn_local(async move {
            for r in (0..bookmarks.row_count()).rev() {
                if let Some(bookmark) = bookmarks.row_data(r) {
                    if !Path::new(bookmark.path()).exists() && repository.remove_bookmark(r).is_ok()
                    {
                        bookmarks.remove(r);
                    }
                }
            }
        });
    }

    fn load_bookmarks(&self) {
        let repository = self.repository.clone();
        let bookmarks = self.bookmarks.clone();

        bookmarks.set_vec(repository.bookmarks());
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
    fn test_reorder() {
        let controller = controller();
        let bookmarks_model = controller.bookmarks();
        let repository = BookmarksRepositoryMock::new(bookmarks());

        repository.reorder(0, 2).unwrap();
        controller.reorder(0, 2);
        let bookmarks = repository.bookmarks();
        assert_model(&bookmarks_model, &bookmarks);

        repository.reorder(3, 1).unwrap();
        controller.reorder(3, 1);
        let bookmarks = repository.bookmarks();
        assert_model(&bookmarks_model, &bookmarks);

        repository.reorder(3, 3).unwrap();
        controller.reorder(3, 3);
        let bookmarks = repository.bookmarks();
        assert_model(&bookmarks_model, &bookmarks);
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

        controller.execute_item_context_menu_action(0, context_menu::REMOVE_BOOKMARK);
        assert_eq!(
            bookmarks_model.row_data(0),
            Some(BookmarkModel::new(
                crate::models::BookmarkType::Game,
                "Games",
                "/games",
                false
            ))
        );

        controller.execute_item_context_menu_action(2, context_menu::REMOVE_BOOKMARK);
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
        let context_menu_model = controller.get_item_context_menu(0);

        assert_eq!(
            &context_menu_model.row_data(0).unwrap().spec,
            context_menu::REMOVE_BOOKMARK
        );

        let context_menu_model = controller.get_item_context_menu(3);
        assert_eq!(context_menu_model.row_count(), 0);
    }

    #[test]
    fn test_open_dir() {
        let controller = controller();

        controller.on_open_internal(|file_model| {
            assert_eq!(file_model.path(), "/documents");
        });
        controller.open_dir(0);
    }

    #[test]
    fn test_open_next_dir() {
        let controller = controller();

        controller.on_open_internal(|file_model| {
            assert_eq!(file_model.path(), "/games");
        });
        controller.select("/documents");
        controller.open_next_dir();
    }

    #[test]
    fn test_open_previous_dir() {
        let controller = controller();

        controller.on_open_internal(|file_model| {
            assert_eq!(file_model.path(), "/pictures");
        });
        controller.select("/");
        controller.open_previous_dir();
    }
}
