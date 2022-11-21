// SPDX-FileCopyrightText: 2022 Florian Blasius <co_sl@tutanota.com>
// SPDX-License-Identifier: GPL-3.0-only

#![allow(clippy::redundant_clone)]
#![allow(clippy::cmp_owned)]

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

use std::rc::Rc;

slint::include_modules!();

use slint::*;

#[cfg_attr(target_arch = "wasm32", wasm_bindgen(start))]
pub fn main() {
    // This provides better error messages in debug mode.
    // It's disabled in release mode so it doesn't bloat up the file size.
    #[cfg(all(debug_assertions, target_arch = "wasm32"))]
    console_error_panic_hook::set_once();

    let ui = App::new();

    let public_icon = ui.global::<mi>().get_public();
    let lock_icon = ui.global::<mi>().get_lock();
    let account_box = ui.global::<mi>().get_account_box();

    let channels = Rc::new(VecModel::default());

    // group channels,
    let group_channels = Rc::new(VecModel::default());
    group_channels.push(ItemModel {
        text: "Town Square".into(),
        leading_icon: public_icon.clone(),
        ..Default::default()
    });
    group_channels.push(ItemModel {
        text: "Internal".into(),
        leading_icon: lock_icon.clone(),
        ..Default::default()
    });
    channels.push(ParentItemModel {
        text: "Channels".into(),
        items: group_channels.into(),
    });

    // direct channels
    let direct_channels = Rc::new(VecModel::default());
    direct_channels.push(ItemModel {
        text: "Simon".into(),
        leading_icon: account_box.clone(),
        ..Default::default()
    });
    direct_channels.push(ItemModel {
        text: "Olivier".into(),
        leading_icon: account_box.clone(),
        ..Default::default()
    });
    direct_channels.push(ItemModel {
        text: "Tobias".into(),
        leading_icon: account_box.clone(),
        ..Default::default()
    });
    direct_channels.push(ItemModel {
        text: "Auri".into(),
        leading_icon: account_box.clone(),
        ..Default::default()
    });
    channels.push(ParentItemModel {
        text: "DIRECT MESSAGES".into(),
        items: direct_channels.into(),
    });

    ui.global::<AppSideBarBridge>()
        .set_channels(channels.into());

    ui.run();
}
