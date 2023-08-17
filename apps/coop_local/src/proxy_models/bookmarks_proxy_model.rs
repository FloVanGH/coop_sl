use std::rc::Rc;

// SPDX-FileCopyrightText: 2023 Florian Blasius <co_sl@tutanota.com>
// SPDX-License-Identifier: GPL-3.0-only
use slint::*;

use crate::{
    model::{BookmarkModel, BookmarkType},
    ui,
};

pub struct BookmarksProxyModel {
    source: Rc<VecModel<BookmarkModel>>,
    window_handle: Weak<ui::MainWindow>,
}

impl BookmarksProxyModel {
    pub fn new(bookmarks: Vec<BookmarkModel>, window_handle: Weak<ui::MainWindow>) -> Self {
        Self {
            source: Rc::new(VecModel::from(bookmarks)),
            window_handle,
        }
    }

    pub fn push(&self, bookmark: BookmarkModel) {
        self.source.push(bookmark);
    }

    pub fn row_data_as_bookmark(&self, row: usize) -> Option<BookmarkModel> {
        self.source.row_data(row)
    }

    pub fn remove(&self, bookmark: BookmarkModel) -> Option<BookmarkModel> {
        for i in 0..self.source.row_count() {
            if let Some(source_bookmark) = self.source.row_data(i) {
                if !source_bookmark.eq(&bookmark) {
                    continue;
                }
            }

            return Some(self.source.remove(i));
        }

        None
    }
}

impl Model for BookmarksProxyModel {
    type Data = ui::ListViewItem;

    fn row_count(&self) -> usize {
        self.source.row_count()
    }

    fn row_data(&self, row: usize) -> Option<Self::Data> {
        if let Some(row_data) = self.source.row_data(row) {
            return Some(ui::ListViewItem {
                leading_icon: bookmark_type_to_icon(&self.window_handle, row_data.bookmark_type()),
                text: row_data.name().into(),
                highlighted: true,
                ..Default::default()
            });
        }

        None
    }

    fn model_tracker(&self) -> &dyn slint::ModelTracker {
        self.source.model_tracker()
    }

    fn as_any(&self) -> &dyn core::any::Any {
        self
    }
}

fn bookmark_type_to_icon(
    window_handle: &Weak<ui::MainWindow>,
    bookmark_type: BookmarkType,
) -> Image {
    if let Some(main_window) = window_handle.upgrade() {
        return match bookmark_type {
            BookmarkType::Root => main_window.global::<ui::Icons>().get_computer(),
            BookmarkType::Dir => main_window.global::<ui::Icons>().get_folder(),
            BookmarkType::Game => main_window.global::<ui::Icons>().get_videogame_asset(),
        };
    }

    Image::default()
}
