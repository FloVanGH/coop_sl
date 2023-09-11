// SPDX-FileCopyrightText: 2023 Florian Blasius <co_sl@tutanota.com>
// SPDX-License-Identifier: GPL-3.0-only

use crate::ui::*;
use slint::*;

#[derive(Clone)]
pub struct DialogController {
    view_handle: Weak<MainWindow>,
}

impl DialogController {
    pub fn new(view_handle: Weak<MainWindow>) -> Self {
        let controller = Self { view_handle };

        upgrade_adapter(&controller.view_handle, {
            let controller = controller.clone();

            move |adapter| {
                adapter.on_close_about({
                    let controller = controller.clone();
                    move || controller.on_close_about()
                });
            }
        });

        controller
    }

    pub fn show_about(&self) {
        upgrade_adapter(&self.view_handle, |adapter| {
            adapter.set_show_about(true);
        });
    }

    fn on_close_about(&self) {
        upgrade_adapter(&self.view_handle, |adapter| {
            adapter.set_show_about(false);
        });
    }
}

fn upgrade_adapter(view_handle: &Weak<MainWindow>, func: impl FnOnce(DialogAdapter) + 'static) {
    if let Some(view) = view_handle.upgrade() {
        func(view.global::<DialogAdapter>());
    }
}
