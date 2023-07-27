// SPDX-FileCopyrightText: 2023 Florian Blasius <co_sl@tutanota.com>
// SPDX-License-Identifier: GPL-3.0-only

use super::ItemsRepository;
use crate::model::{self, ItemModel};
use std::sync::{Arc, Mutex};

#[derive(Clone)]
pub struct ItemsRepositoryMock {
    current_items: Arc<Mutex<Vec<Vec<ItemModel>>>>,
    root_titles: Arc<Mutex<Vec<String>>>,
}

impl ItemsRepositoryMock {
    pub fn new() -> Self {
        Self {
            current_items: Arc::new(Mutex::new(vec![])),
            root_titles: Arc::new(Mutex::new(vec![])),
        }
    }
}

impl ItemsRepository for ItemsRepositoryMock {
    fn items(&self, root: String) -> Vec<model::ItemModel> {
        if let Ok(mut root_titles) = self.root_titles.lock() {
            root_titles.push(root.clone());
        }

        let items = match root.as_str() {
            "/docs" => {
                let mut items = vec![ItemModel::new(
                    model::ItemType::Folder,
                    "basics",
                    "/docs/basics",
                )];

                items.append(
                    &mut (0..500)
                        .map(|i| {
                            model::ItemModel::new(
                                model::ItemType::Text,
                                format!("file_{}.md", i),
                                format!("/docs/file_{}.md", i),
                            )
                        })
                        .collect(),
                );
                items
            }
            "/docs/basics" => (0..500)
                .map(|i| {
                    model::ItemModel::new(
                        model::ItemType::Text,
                        format!("file_{}.md", i),
                        format!("/docs/basics/file_{}.md", i),
                    )
                })
                .collect(),
            "/src" => (0..500)
                .map(|i| {
                    model::ItemModel::new(
                        model::ItemType::Text,
                        format!("file_{}.slint", i),
                        format!("/src/file_{}.slint", i),
                    )
                })
                .collect(),
            _ => vec![
                model::ItemModel::new(model::ItemType::Folder, "docs", "/docs"),
                model::ItemModel::new(model::ItemType::Folder, "src", "/src"),
                model::ItemModel::new(model::ItemType::Text, "README.md", "/README-md"),
            ],
        };

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
        if let Ok(mut root_titles) = self.root_titles.lock() {
            return root_titles.last().cloned();
        }

        None
    }
}
