// SPDX-FileCopyrightText: 2023 Florian Blasius <co_sl@tutanota.com>
// SPDX-License-Identifier: GPL-3.0-only

use std::rc::Rc;

use crate::{
    controllers::TextEditorController, models::TextModel,
    repositories::traits::TextEditorRepository, ui::*,
};
use slint::*;

pub fn connect_with_controller<T: TextEditorRepository>(
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

pub fn connect<T: TextEditorRepository + Clone + Send + Sync + 'static>(
    view_handle: &Weak<MainWindow>,
    controller: &Rc<TextEditorController<T>>,
) {
    connect_with_controller(view_handle, controller, |adapter, controller| {
        adapter.on_update_text(move |text| controller.update_text(text.as_str()));
    });

    connect_with_controller(view_handle, controller, |adapter, controller| {
        adapter.on_get_context_menu(move || controller.context_menu());
    });

    connect_with_controller(view_handle, controller, |adapter, controller| {
        adapter.on_context_menu_action({
            move |spec| controller.execute_context_menu_action(spec.as_str(), true)
        });
    });

    connect_with_controller(view_handle, controller, |adapter, controller| {
        adapter.on_next(move || controller.next(true));
    });

    connect_with_controller(view_handle, controller, |adapter, controller| {
        adapter.on_previous(move || controller.previous(true));
    });

    connect_with_controller(view_handle, controller, |adapter, controller| {
        adapter.on_back(move || controller.invoke_back());
    });

    controller.on_loading_changed({
        let view_handle = view_handle.clone();
        move |loading| {
            set_loading(&view_handle, *loading);
        }
    });

    controller.on_single_text_changed({
        let view_handle = view_handle.clone();
        move |is_single_text| {
            set_is_single_text(&view_handle, *is_single_text);
        }
    });

    controller.on_current_model_changed({
        let view_handle = view_handle.clone();
        move |model| {
            set_model(&view_handle, model);
        }
    });
}
