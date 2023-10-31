// SPDX-FileCopyrightText: 2023 Florian Blasius <co_sl@tutanota.com>
// SPDX-License-Identifier: GPL-3.0-only

use crate::{controllers::FilesController, ui::*};
use slint::*;

pub fn connect(
    view_handle: &Weak<MainWindow>,
    controller: &FilesController,
    func: impl FnOnce(FilesAdapter, FilesController) + 'static,
) {
    if let Some(view) = view_handle.upgrade() {
        func(view.global::<FilesAdapter>(), controller.clone());
    }
}

pub fn get(view_handle: &Weak<MainWindow>, func: impl FnOnce(FilesAdapter) + 'static) {
    if let Some(view) = view_handle.upgrade() {
        func(view.global::<FilesAdapter>());
    }
}
