// SPDX-FileCopyrightText: 2023 Florian Blasius <co_sl@tutanota.com>
// SPDX-License-Identifier: GPL-3.0-only

use crate::model::ItemModel;

pub trait ItemsRepository {
    fn items(&self, root: String) -> Vec<ItemModel>;
    fn item(&self, page_index: usize, item_index: usize) -> Option<ItemModel>;
    fn current_root_title(&self) -> Option<String>;
    fn pop(&self);
}
