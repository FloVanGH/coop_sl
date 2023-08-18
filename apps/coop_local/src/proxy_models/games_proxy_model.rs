use std::rc::Rc;

// SPDX-FileCopyrightText: 2023 Florian Blasius <co_sl@tutanota.com>
// SPDX-License-Identifier: GPL-3.0-only
use slint::*;

use crate::{model::GameModel, ui};

pub struct GamesProxyModel {
    source: Rc<VecModel<GameModel>>,
    wrapped_model: ModelRc<GameModel>,
}

impl GamesProxyModel {
    pub fn new(games: Vec<GameModel>) -> Self {
        let source = Rc::new(VecModel::from(games));
        Self {
            source: source.clone(),
            wrapped_model: source.into(),
        }
    }

    pub fn push_to_source(&self, game: GameModel) {
        self.source.push(game);
    }

    pub fn row_data_as_game(&self, row: usize) -> Option<GameModel> {
        self.wrapped_model.row_data(row)
    }

    pub fn set_row_data_as_game(&self, row: usize, data: GameModel) {
        self.wrapped_model.set_row_data(row, data);
    }

    pub fn remove_from_source(&self, game: GameModel) -> Option<GameModel> {
        for i in 0..self.source.row_count() {
            if let Some(source_bookmark) = self.source.row_data(i) {
                if !source_bookmark.eq(&game) {
                    continue;
                }
            }

            return Some(self.source.remove(i));
        }

        None
    }

    pub fn row(&self, game: &GameModel) -> Option<usize> {
        for row in 0..self.row_count() {
            if let Some(g) = self.row_data_as_game(row) {
                if g.eq(game) {
                    return Some(row);
                }
            }
        }

        None
    }

    pub fn as_sort_by<F>(&self, sort_function: F) -> Self
    where
        F: FnMut(&GameModel, &GameModel) -> core::cmp::Ordering + 'static,
    {
        Self {
            source: self.source.clone(),
            wrapped_model: Rc::new(self.wrapped_model.clone().sort_by(sort_function)).into(),
        }
    }
}

impl Model for GamesProxyModel {
    type Data = ui::LauncherItem;

    fn row_count(&self) -> usize {
        self.wrapped_model.row_count()
    }

    fn row_data(&self, row: usize) -> Option<Self::Data> {
        if let Some(row_data) = self.wrapped_model.row_data(row) {
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
        self.wrapped_model.model_tracker()
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
