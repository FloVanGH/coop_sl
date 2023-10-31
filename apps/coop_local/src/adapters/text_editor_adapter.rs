// SPDX-FileCopyrightText: 2023 Florian Blasius <co_sl@tutanota.com>
// SPDX-License-Identifier: GPL-3.0-only

use std::rc::Rc;

use crate::{
    controllers::TextEditorController, models::TextModel,
    repositories::traits::TextEditorRepository, ui::*,
};
use slint::*;

pub fn connect<T: TextEditorRepository>(
    view_handle: &Weak<MainWindow>,
    controller: &Rc<TextEditorController<T>>,
    func: impl FnOnce(TextEditorAdapter, Rc<TextEditorController<T>>) + 'static,
) {
    if let Some(view) = view_handle.upgrade() {
        func(view.global::<TextEditorAdapter>(), controller.clone());
    }
}

pub fn set_loading(view_handle: &Weak<MainWindow>, loading: bool) {
    get(view_handle, move |adapter| {
        adapter.set_loading(loading);
    });
}

pub fn set_model(view_handle: &Weak<MainWindow>, model: &TextModel) {
    let model = model.clone();
    get(view_handle, move |adapter| {
        adapter.set_title(model.title().into());
        adapter.set_text(model.text_update().into());
        adapter.set_read_only(model.readonly());
        adapter.set_has_changes(model.has_changes());
        adapter.set_loading(false);
    });
}

pub fn set_is_single_text(view_handle: &Weak<MainWindow>, single_text: bool) {
    get(view_handle, move |adapter| {
        adapter.set_is_single_text(single_text);
    });
}

pub fn get(view_handle: &Weak<MainWindow>, func: impl FnOnce(TextEditorAdapter) + 'static) {
    if let Some(view) = view_handle.upgrade() {
        func(view.global::<TextEditorAdapter>());
    }
}
