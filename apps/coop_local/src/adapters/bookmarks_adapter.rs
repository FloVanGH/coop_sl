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

pub fn connect<T: BookmarksRepository>(
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
