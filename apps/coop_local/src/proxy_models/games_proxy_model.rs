use std::rc::Rc;

// SPDX-FileCopyrightText: 2023 Florian Blasius <co_sl@tutanota.com>
// SPDX-License-Identifier: GPL-3.0-only
use slint::*;

use crate::{model::GameModel, ui};

pub struct GamesProxyModel {
    source: Rc<VecModel<GameModel>>,
}

impl GamesProxyModel {
    pub fn new(games: Vec<GameModel>) -> Self {
        Self {
            source: Rc::new(VecModel::from(games)),
        }
    }

    pub fn push(&self, bookmark: GameModel) {
        self.source.push(bookmark);
    }

    pub fn row_data_as_game(&self, row: usize) -> Option<GameModel> {
        self.source.row_data(row)
    }

    pub fn set_row_data_as_game(&self, row: usize, data: GameModel) {
        self.source.set_row_data(row, data);
    }

    pub fn remove(&self, bookmark: GameModel) -> Option<GameModel> {
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

impl Model for GamesProxyModel {
    type Data = ui::LauncherItem;

    fn row_count(&self) -> usize {
        self.source.row_count()
    }

    fn row_data(&self, row: usize) -> Option<Self::Data> {
        if let Some(row_data) = self.source.row_data(row) {
            let image = if row_data.icon().width() == 0 || row_data.icon().height() == 0 {
                Image::default()
            } else {
                Image::from_rgba8(SharedPixelBuffer::<Rgba8Pixel>::clone_from_slice(
                    row_data.icon().data(),
                    row_data.icon().width(),
                    row_data.icon().height(),
                ))
            };

            return Some(ui::LauncherItem {
                image,
                text: row_data.name().into(),
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

// fn bookmark_type_to_icon(
//     window_handle: &Weak<ui::MainWindow>,
//     bookmark_type: BookmarkType,
// ) -> Image {
//     if let Some(main_window) = window_handle.upgrade() {
//         return match bookmark_type {
//             BookmarkType::Root => main_window.global::<ui::Icons>().get_computer(),
//             BookmarkType::Dir => main_window.global::<ui::Icons>().get_folder(),
//         };
//     }

//     Image::default()
// }
