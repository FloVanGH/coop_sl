// SPDX-FileCopyrightText: 2023 Florian Blasius <co_sl@tutanota.com>
// SPDX-License-Identifier: GPL-3.0-only

use super::{SideBarController, SideBarMessage};
use crate::ui;
use slint::{ComponentHandle, ModelRc, VecModel};
use std::rc::Rc;

const REMOVE: &str = "remove";

pub fn on_context_menu_action(controller: SideBarController, main_window: &ui::MainWindow) {
    main_window
        .global::<ui::SideBarAdapter>()
        .on_context_menu_action(move |_parent_index, bookmark_index, action| {
            if action.as_str() == REMOVE {
                controller.spawn_message(SideBarMessage::RemoveBookmark {
                    bookmark: bookmark_index as usize,
                })
            }
        });
}

pub fn get_context_menu(parent: usize) -> ModelRc<ui::ListViewItem> {
    if parent > 0 {
        return Rc::new(VecModel::default()).into();
    }

    let items = VecModel::default();

    items.push(ui::ListViewItem {
        text: "Remove".into(),
        spec: REMOVE.into(),
        ..Default::default()
    });

    Rc::new(items).into()
}
