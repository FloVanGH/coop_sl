// SPDX-FileCopyrightText: 2023 Florian Blasius <co_sl@tutanota.com>
// SPDX-License-Identifier: GPL-3.0-only

use std::rc::Rc;

use crate::{
    controllers::BookmarksController,
    models::{BookmarkModel, BookmarkType},
    repositories::traits::BookmarksRepository,
    ui::*,
};
use slint::*;

pub fn connect_with_controller<T: BookmarksRepository>(
    view_handle: &Weak<MainWindow>,
    controller: &Rc<BookmarksController<T>>,
    func: impl FnOnce(BookmarksAdapter, Rc<BookmarksController<T>>) + 'static,
) {
    if let Some(view) = view_handle.upgrade() {
        func(view.global::<BookmarksAdapter>(), controller.clone());
    }
}

pub fn map_bookmarks(
    view_handle: Weak<MainWindow>,
    model: ModelRc<BookmarkModel>,
) -> ModelRc<ListViewItem> {
    Rc::new(model.map(move |b: BookmarkModel| ListViewItem {
        leading_icon: bookmark_type_to_icon(&view_handle, b.bookmark_type()),
        text: b.name().into(),
        highlighted: !b.unremovable(),
        selected: b.selected(),
        ..Default::default()
    }))
    .into()
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

pub fn connect<T: BookmarksRepository + 'static>(
    view_handle: &Weak<MainWindow>,
    controller: &Rc<BookmarksController<T>>,
) {
    connect_with_controller(view_handle, controller, {
        let view_handle = view_handle.clone();
        move |adapter, controller| {
            adapter.set_bookmarks(map_bookmarks(view_handle, controller.bookmarks()));
        }
    });
    connect_with_controller(view_handle, controller, |adapter, controller| {
        adapter.on_open_dir(move |item| {
            controller.open(item as usize);
        });
    });
    connect_with_controller(view_handle, controller, |adapter, controller| {
        adapter.on_open_next_dir(move || {
            controller.next();
        });
    });
    connect_with_controller(view_handle, controller, |adapter, controller| {
        adapter.on_open_previous_dir(move || {
            controller.previous();
        });
    });
    connect_with_controller(view_handle, controller, |adapter, controller| {
        adapter.on_context_menu(move |index| controller.context_menu(index as usize));
    });
    connect_with_controller(view_handle, controller, |adapter, controller| {
        adapter.on_context_menu_action(move |index, spec| {
            controller.execute_context_menu_action(index as usize, spec.as_str());
        });
    });
    connect_with_controller(view_handle, controller, |adapter, controller| {
        adapter.on_selected_item(move || controller.selected_item());
    });
    connect_with_controller(view_handle, controller, |adapter, controller| {
        adapter.on_drop(move |source, target| {
            controller.execute_drop(source as usize, target as usize);
        });
    });
}
