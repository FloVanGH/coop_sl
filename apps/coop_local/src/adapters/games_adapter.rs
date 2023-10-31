// SPDX-FileCopyrightText: 2023 Florian Blasius <co_sl@tutanota.com>
// SPDX-License-Identifier: GPL-3.0-only

use crate::{controllers::GamesController, ui::*};
use slint::*;

pub fn connect(
    view_handle: &Weak<MainWindow>,
    controller: &GamesController,
    func: impl FnOnce(GamesAdapter, GamesController) + 'static,
) {
    if let Some(view) = view_handle.upgrade() {
        func(view.global::<GamesAdapter>(), controller.clone());
    }
}

pub fn get(view_handle: &Weak<MainWindow>, func: impl FnOnce(GamesAdapter) + 'static) {
    if let Some(view) = view_handle.upgrade() {
        func(view.global::<GamesAdapter>());
    }
}
