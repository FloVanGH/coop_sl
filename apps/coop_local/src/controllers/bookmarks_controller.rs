// SPDX-FileCopyrightText: 2023 Florian Blasius <co_sl@tutanota.com>
// SPDX-License-Identifier: GPL-3.0-only

use std::cell::RefCell;
use std::path::Path;
use std::rc::Rc;
use std::{cell::Cell, io};

use crate::models::FileModel;
use crate::{
    models::{BookmarkModel, BookmarkType},
    repositories::BookmarksRepository,
    services::SettingsService,
    ui::*,
};
use slint::*;

mod context_menu {
    pub const REMOVE_BOOKMARK: &str = "remove bookmark";
}

type FileCallback = Rc<RefCell<Box<dyn FnMut(FileModel) + 'static>>>;

#[derive(Clone)]
pub struct BookmarksController {
    bookmarks: Rc<VecModel<BookmarkModel>>,
    repository: BookmarksRepository,
    selected_bookmark: Rc<Cell<Option<usize>>>,
    view_handle: Weak<MainWindow>,
    open_internal_callback: FileCallback,
}

impl BookmarksController {
    pub fn new(view_handle: Weak<MainWindow>) -> io::Result<Self> {
        let bookmarks = Rc::new(VecModel::default());

        let view_handle_copy = view_handle.clone();

        upgrade_adapter(&view_handle, {
            let bookmarks = bookmarks.clone();

            move |adapter| {
                adapter.set_bookmarks(
                    Rc::new(bookmarks.map(move |b: BookmarkModel| ListViewItem {
                        leading_icon: bookmark_type_to_icon(&view_handle_copy, b.bookmark_type()),
                        text: b.name().into(),
                        highlighted: !b.unremovable(),
                        selected: b.selected(),
                        ..Default::default()
                    }))
                    .into(),
                );
            }
        });

        let repository = BookmarksRepository::new(SettingsService::new()?);

        let controller = Self {
            bookmarks,
            view_handle,
            repository,
            selected_bookmark: Rc::new(Cell::new(None)),
            open_internal_callback: Rc::new(RefCell::new(Box::new(|_f: FileModel| {}))),
        };

        controller.load_bookmarks();

        upgrade_adapter(&controller.view_handle, {
            let controller = controller.clone();

            move |adapter| {
                adapter.on_open_dir({
                    let controller = controller.clone();
                    move |item| {
                        controller.on_open_dir(item as usize);
                    }
                });

                adapter.on_open_next_dir({
                    let controller = controller.clone();
                    move || {
                        controller.open_next_dir();
                    }
                });

                adapter.on_open_previous_dir({
                    let controller = controller.clone();
                    move || {
                        controller.open_previous_dir();
                    }
                });

                adapter.on_get_item_context_menu({
                    let controller = controller.clone();
                    move |index| controller.get_item_context_menu(index as usize)
                });

                adapter.on_item_context_menu_action({
                    let controller = controller.clone();
                    move |index, spec| {
                        controller.execute_item_context_menu_action(index as usize, spec.as_str())
                    }
                });

                adapter.on_selected_item({
                    let controller = controller.clone();
                    move || controller.selected_item()
                });

                adapter.on_reorder({
                    let controller = controller.clone();
                    move |source, target| controller.reorder(source as usize, target as usize)
                })
            }
        });

        Ok(controller)
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

    pub fn on_open_internal(&self, callback: impl FnMut(FileModel) + 'static) {
        *self.open_internal_callback.borrow_mut() = Box::new(callback);
    }

    pub fn add_bookmark(&self, bookmark: BookmarkModel) {
        if self.bookmarks.iter().any(|b| b.eq(&bookmark)) {
            return;
        }

        let repository = self.repository.clone();
        let bookmarks = self.bookmarks.clone();

        let _ = slint::spawn_local(async move {
            if repository.add_bookmark(&bookmark).is_ok() {
                bookmarks.push(bookmark);
            }
        });
    }

    pub fn get_first_bookmark(&self) -> Option<String> {
        if self.bookmarks.row_count() > 0 {
            return self.bookmarks.row_data(0).map(|b| b.path().to_string());
        }

        None
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

    fn execute_item_context_menu_action(&self, index: usize, spec: &str) {
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

    fn get_item_context_menu(&self, index: usize) -> ModelRc<ListViewItem> {
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

    fn load_bookmarks(&self) {
        let repository = self.repository.clone();
        let bookmarks = self.bookmarks.clone();

        bookmarks.set_vec(repository.bookmarks());
    }

    fn on_open_dir(&self, item: usize) {
        if let Some(bookmark) = self.bookmarks.row_data(item) {
            let mut r: std::cell::RefMut<'_, Box<dyn FnMut(FileModel)>> =
                self.open_internal_callback.try_borrow_mut().unwrap();
            r(FileModel::new(bookmark.path()));
        }
    }

    fn open_next_dir(&self) {
        if let Some(selected_bookmark) = self.selected_bookmark.get() {
            if selected_bookmark + 1 < self.bookmarks.row_count() {
                self.on_open_dir(selected_bookmark + 1);
            } else {
                self.on_open_dir(0);
            }
        }
    }

    fn open_previous_dir(&self) {
        if let Some(selected_bookmark) = self.selected_bookmark.get() {
            if selected_bookmark > 0 {
                self.on_open_dir(selected_bookmark - 1);
            }
        }
    }

    fn selected_item(&self) -> i32 {
        if let Some(selected_item) = self.selected_bookmark.get() {
            return selected_item as i32;
        }

        -1
    }

    fn reorder(&self, source: usize, target: usize) {
        if let Ok(reorder) = self.repository.reorder(source, target) {
            if reorder {
                self.bookmarks.set_vec(self.repository.bookmarks());
            }
        }
    }
}

fn upgrade_adapter(view_handle: &Weak<MainWindow>, func: impl FnOnce(BookmarksAdapter) + 'static) {
    if let Some(view) = view_handle.upgrade() {
        func(view.global::<BookmarksAdapter>());
    }
}

fn bookmark_type_to_icon(view_handle: &Weak<MainWindow>, bookmark_type: BookmarkType) -> Image {
    if let Some(main_window) = view_handle.upgrade() {
        return match bookmark_type {
            BookmarkType::Root => main_window.global::<Icons>().get_computer(),
            BookmarkType::Dir => main_window.global::<Icons>().get_folder(),
            BookmarkType::Game => main_window.global::<Icons>().get_videogame_asset(),
            BookmarkType::Volume => main_window.global::<Icons>().get_storage(),
        };
    }

    Image::default()
}
