// SPDX-FileCopyrightText: 2023 Florian Blasius <co_sl@tutanota.com>
// SPDX-License-Identifier: GPL-3.0-only

use super::ItemsRepository;
use crate::model::{self, ItemModel};
use std::fs;
use std::path::Path;
use std::sync::{Arc, Mutex};

#[derive(Clone)]
pub struct FileItemsRepository {
    current_items: Arc<Mutex<Vec<Vec<ItemModel>>>>,
    root_titles: Arc<Mutex<Vec<String>>>,
}

impl FileItemsRepository {
    pub fn new() -> Self {
        Self {
            current_items: Arc::new(Mutex::new(vec![])),
            root_titles: Arc::new(Mutex::new(vec![])),
        }
    }
}

impl ItemsRepository for FileItemsRepository {
    fn items(&self, root: String) -> Vec<model::ItemModel> {
        let mut items = vec![];

        if let Ok(mut root_titles) = self.root_titles.lock() {
            if let Some(file_name) = Path::new(root.as_str()).file_name() {
                root_titles.push(file_name.to_str().unwrap_or_default().to_string());
            } else {
                root_titles.push(root.clone());
            }
        }

        // FIXME: handle errors

        if let Ok(paths) = fs::read_dir(root) {
            for entry in paths.flatten() {
                let file_name = entry.file_name().to_str().unwrap_or_default().to_string();
                let path = entry.path().to_str().unwrap_or_default().to_string();

                // FIXME: check types for images, text, ...
                let file_type = if let Ok(file_type) = entry.file_type() {
                    if file_type.is_dir() {
                        model::ItemType::Folder
                    } else {
                        model::ItemType::Text
                    }
                } else {
                    model::ItemType::Unknown
                };

                items.push(ItemModel::new(file_type, file_name, path));
            }
        }

        items.sort_by(|l, r| {
            if l.item_type == r.item_type {
                l.text.cmp(&r.text)
            } else {
                l.item_type.cmp(&r.item_type)
            }
        });

        if let Ok(mut current_items) = self.current_items.lock() {
            current_items.push(items.clone());
        }

        items
    }

    fn item(&self, page_index: usize, item_index: usize) -> Option<model::ItemModel> {
        if let Ok(current_items) = self.current_items.lock() {
            if let Some(items) = current_items.get(page_index) {
                return items.get(item_index).cloned();
            }
        }

        None
    }

    fn pop(&self) {
        if let Ok(mut current_items) = self.current_items.lock() {
            current_items.pop();
        }

        if let Ok(mut root_titles) = self.root_titles.lock() {
            root_titles.pop();
        }
    }

    fn current_root_title(&self) -> Option<String> {
        if let Ok(root_titles) = self.root_titles.lock() {
            return root_titles.last().cloned();
        }

        None
    }
}
