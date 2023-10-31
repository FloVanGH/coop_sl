// SPDX-FileCopyrightText: 2023 Florian Blasius <co_sl@tutanota.com>
// SPDX-License-Identifier: GPL-3.0-only

use std::rc::Rc;

use crate::{controllers::ImageController, ui::*};
use slint::*;

pub fn connect(
    view_handle: &Weak<MainWindow>,
    controller: &Rc<ImageController>,
    func: impl FnOnce(ImageAdapter, Rc<ImageController>) + 'static,
) {
    if let Some(view) = view_handle.upgrade() {
        func(view.global::<ImageAdapter>(), controller.clone());
    }
}

pub fn get(view_handle: &Weak<MainWindow>, func: impl FnOnce(ImageAdapter) + 'static) {
    if let Some(view) = view_handle.upgrade() {
        func(view.global::<ImageAdapter>());
    }
}
