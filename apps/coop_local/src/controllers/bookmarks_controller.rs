// SPDX-FileCopyrightText: 2023 Florian Blasius <co_sl@tutanota.com>
// SPDX-License-Identifier: GPL-3.0-only

use std::cell::RefCell;
use std::path::Path;
use std::rc::Rc;
use std::{cell::Cell, io};

use crate::models::FileModel;
use crate::{
    models::{BookmarkModel, BookmarkType},
    proxy_model::ProxyModel,
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
    bookmarks: Rc<ProxyModel<BookmarkModel>>,
    locations: Rc<ProxyModel<BookmarkModel>>,
    repository: BookmarksRepository,
    selected_bookmark: Rc<Cell<Option<usize>>>,
    selected_location: Rc<Cell<Option<usize>>>,
    view_handle: Weak<MainWindow>,
    open_internal_callback: FileCallback,
}

impl BookmarksController {
    pub fn new(view_handle: Weak<MainWindow>) -> io::Result<Self> {
        let bookmarks = Rc::new(ProxyModel::default());
        let locations = Rc::new(ProxyModel::default());

        let view_handle_copy = view_handle.clone();
        let view_handle_copy_locations = view_handle.clone();

        upgrade_adapter(&view_handle, {
            let bookmarks = bookmarks.clone();
            let locations = locations.clone();

            move |adapter| {
                adapter.set_bookmarks(VecModel::from_slice(&[
                    GroupListViewItem {
                        text: "Bookmarks".into(),
                        items: Rc::new(bookmarks.clone().map(move |b: BookmarkModel| {
                            ListViewItem {
                                leading_icon: bookmark_type_to_icon(
                                    &view_handle_copy,
                                    b.bookmark_type(),
                                ),
                                text: b.name().into(),
                                highlighted: true,
                                selected: b.selected(),
                                ..Default::default()
                            }
                        }))
                        .into(),
                    },
                    GroupListViewItem {
                        text: "Locations".into(),
                        items: Rc::new(locations.clone().map(move |b: BookmarkModel| {
                            ListViewItem {
                                leading_icon: bookmark_type_to_icon(
                                    &view_handle_copy_locations,
                                    b.bookmark_type(),
                                ),
                                text: b.name().into(),
                                highlighted: true,
                                selected: b.selected(),
                                ..Default::default()
                            }
                        }))
                        .into(),
                    },
                ]));
            }
        });

        let repository = BookmarksRepository::new(SettingsService::new()?);

        let controller = Self {
            bookmarks,
            locations,
            view_handle,
            repository,
            selected_bookmark: Rc::new(Cell::new(None)),
            selected_location: Rc::new(Cell::new(None)),
            open_internal_callback: Rc::new(RefCell::new(Box::new(|_f: FileModel| {}))),
        };

        controller.load_bookmarks();

        upgrade_adapter(&controller.view_handle, {
            let controller = controller.clone();

            move |adapter| {
                adapter.on_open_dir({
                    let controller = controller.clone();
                    move |parent, item| {
                        controller.on_open_dir(parent as usize, item as usize);
                    }
                });
                adapter.on_get_item_context_menu({
                    let controller = controller.clone();
                    move |parent_row, item_row| {
                        controller.get_item_context_menu(parent_row as usize, item_row as usize)
                    }
                });
                adapter.on_item_context_menu_action({
                    let controller = controller.clone();
                    move |parent_row, item_row, spec| {
                        controller.execute_item_context_menu_action(
                            parent_row as usize,
                            item_row as usize,
                            spec.as_str(),
                        )
                    }
                });
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

        if let Some(selected_location) = self.selected_location.get() {
            if let Some(mut locations) = self.locations.row_data(selected_location) {
                locations.set_selected(false);
                self.locations.set_row_data(selected_location, locations);
            }

            self.selected_location.set(None);
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

        for r in 0..self.locations.row_count() {
            if let Some(mut location) = self.locations.row_data(r) {
                if location.path().eq(path) {
                    location.set_selected(true);
                    self.locations.set_row_data(r, location);
                    self.selected_location.set(Some(r));
                    return;
                }
            }
        }
    }

    pub fn on_open_internal(&self, callback: impl FnMut(FileModel) + 'static) {
        *self.open_internal_callback.borrow_mut() = Box::new(callback);
    }

    pub fn add_bookmark(&self, bookmark: BookmarkModel) {
        let repository = self.repository.clone();
        let bookmarks = self.bookmarks.clone();

        let _ = slint::spawn_local(async move {
            if repository.add_bookmark(&bookmark).is_ok() {
                bookmarks.push_to_source(bookmark);
            }
        });
    }

    pub fn get_first_bookmark(&self) -> Option<String> {
        if self.bookmarks.row_count() > 0 {
            return self.bookmarks.row_data(0).map(|b| b.path().to_string());
        }

        if self.locations.row_count() > 0 {
            return self.locations.row_data(0).map(|b| b.path().to_string());
        }

        None
    }

    pub fn update(&self) {
        let bookmarks = self.bookmarks.clone();
        let locations = self.locations.clone();
        let repository = self.repository.clone();

        let _ = slint::spawn_local(async move {
            for r in (0..bookmarks.row_count()).rev() {
                if let Some(bookmark) = bookmarks.row_data(r) {
                    if !Path::new(bookmark.path()).exists() && repository.remove_bookmark(r).is_ok()
                    {
                        bookmarks.remove_from_source(bookmark);
                    }
                }
            }

            for r in (0..locations.row_count()).rev() {
                if let Some(location) = locations.row_data(r) {
                    if !Path::new(location.path()).exists() {
                        locations.remove_from_source(location);
                    }
                }
            }
        });
    }

    fn execute_item_context_menu_action(&self, parent_row: usize, item_row: usize, spec: &str) {
        if parent_row != 0 {
            return;
        }

        if !spec.eq(context_menu::REMOVE_BOOKMARK) {
            return;
        }

        if self.repository.remove_bookmark(item_row).is_ok() {
            if let Some(bookmark_model) = self.bookmarks.row_data(item_row) {
                self.bookmarks.remove_from_source(bookmark_model);
            }
        }
    }

    fn get_item_context_menu(&self, parent_row: usize, _item_row: usize) -> ModelRc<ListViewItem> {
        if parent_row != 0 {
            return Rc::new(VecModel::default()).into();
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
        let locations = self.locations.clone();

        bookmarks.set_vec_to_source(repository.bookmarks());
        locations.set_vec_to_source(repository.locations());
    }

    fn on_open_dir(&self, parent: usize, item: usize) {
        if parent > 1 {
            return;
        }

        if parent == 0 {
            if let Some(bookmark) = self.bookmarks.row_data(item) {
                let mut r: std::cell::RefMut<'_, Box<dyn FnMut(FileModel)>> =
                    self.open_internal_callback.try_borrow_mut().unwrap();
                r(FileModel::new(bookmark.path()));
            }
        }

        if parent == 1 {
            if let Some(bookmark) = self.locations.row_data(item) {
                let mut r = self.open_internal_callback.try_borrow_mut().unwrap();
                r(FileModel::new(bookmark.path()));
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
        };
    }

    Image::default()
}
