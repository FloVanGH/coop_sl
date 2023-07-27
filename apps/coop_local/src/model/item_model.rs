// SPDX-FileCopyrightText: 2023 Florian Blasius <co_sl@tutanota.com>
// SPDX-License-Identifier: GPL-3.0-only

#[derive(Clone, Debug, PartialEq, PartialOrd, Eq, Ord)]
pub enum ItemType {
    Folder,
    Text,
    // Image,
    Unknown,
}

#[derive(Clone, Debug)]
pub struct ItemModel {
    pub item_type: ItemType,
    pub text: String,
    pub content: String,
}

impl ItemModel {
    pub fn new(item_type: ItemType, text: impl Into<String>, content: impl Into<String>) -> Self {
        ItemModel {
            item_type,
            text: text.into(),
            content: content.into(),
        }
    }
}
