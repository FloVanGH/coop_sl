// SPDX-FileCopyrightText: 2023 Florian Blasius <co_sl@tutanota.com>
// SPDX-License-Identifier: GPL-3.0-only

use crate::model::ItemType;
use crate::{model, repository, ui};
use slint::*;
use std::rc::Rc;
use tokio::runtime::Builder;
use tokio::sync::mpsc;

pub enum ItemsViewMessage {
    DisplayItems { root: String },
    OpenItem { page_index: i32, item_index: i32 },
    Back,
}

#[derive(Clone)]
pub struct ItemsViewController {
    spawn: mpsc::Sender<ItemsViewMessage>,
}

impl ItemsViewController {
    pub fn new<R>(main_window: &ui::MainWindow, repository: R) -> Self
    where
        R: repository::ItemsRepository + Clone + std::marker::Send + 'static,
    {
        let (send, mut recv) = mpsc::channel(16);

        let rt = Builder::new_current_thread().enable_all().build().unwrap();

        let controller = Self { spawn: send };

        main_window
            .global::<ui::ItemsViewAdapter>()
            .on_current_item_change({
                let controller = controller.clone();
                move |page_index, item_index| {
                    controller.spawn_message(ItemsViewMessage::OpenItem {
                        page_index,
                        item_index,
                    });
                }
            });

        main_window.global::<ui::ItemsViewAdapter>().on_back({
            let controller = controller.clone();
            move || {
                controller.spawn_message(ItemsViewMessage::Back);
            }
        });

        main_window
            .global::<ui::ItemsViewAdapter>()
            .set_model(Rc::new(VecModel::default()).into());

        std::thread::spawn({
            let window_handle = main_window.as_weak();
            let repository = repository;

            move || {
                rt.block_on(async move {
                    while let Some(message) = recv.recv().await {
                        tokio::spawn(handle_message(
                            message,
                            repository.clone(),
                            window_handle.clone(),
                        ));
                    }
                });
            }
        });

        controller
    }

    pub fn spawn_message(&self, message: ItemsViewMessage) {
        match self.spawn.blocking_send(message) {
            Ok(()) => {}
            Err(_) => panic!("The shared runtime has shut down."),
        }
    }
}

async fn handle_message<R>(
    message: ItemsViewMessage,
    repository: R,
    window_handle: Weak<ui::MainWindow>,
) where
    R: repository::ItemsRepository + std::marker::Send + 'static,
{
    match message {
        ItemsViewMessage::DisplayItems { root } => {
            tokio::spawn(display_items(
                root.clone(),
                repository.items(root),
                window_handle,
            ));
        }
        ItemsViewMessage::OpenItem {
            page_index,
            item_index,
        } => {
            if page_index < 0 || item_index < 0 {
                return;
            }

            if let Some(item) = repository.item(page_index as usize, item_index as usize) {
                match item.item_type {
                    ItemType::Folder => tokio::spawn(display_items(
                        item.text.clone(),
                        repository.items(item.content),
                        window_handle,
                    )),
                    ItemType::Text => todo!(),
                    // ItemType::Image => todo!(),
                    ItemType::Unknown => todo!(),
                };
            }
        }
        ItemsViewMessage::Back => {
            repository.pop();
            tokio::spawn(back(
                repository.current_root_title().unwrap_or_default(),
                window_handle,
            ));
        }
    };
}

async fn display_items(
    title: String,
    items: Vec<model::ItemModel>,
    window_handle: Weak<ui::MainWindow>,
) {
    window_handle
        .upgrade_in_event_loop(move |main_window| {
            let window_handle = main_window.as_weak();

            let items = VecModel::from_slice(items.as_slice()).map(move |i| ui::ListViewItem {
                text: i.text.into(),
                highlighted: i.item_type == ItemType::Folder,
                leading_icon: item_type_to_icon(window_handle.clone(), i.item_type),
                ..Default::default()
            });

            let model = main_window.global::<ui::ItemsViewAdapter>().get_model();

            if let Some(model) = model
                .as_any()
                .downcast_ref::<VecModel<ModelRc<ui::ListViewItem>>>()
            {
                model.push(Rc::new(items).into());
            }

            let current_count = model.row_count();
            main_window
                .global::<ui::ItemsViewAdapter>()
                .set_current_page(current_count as i32 - 1);

            main_window
                .global::<ui::ItemsViewAdapter>()
                .set_title(title.into());
        })
        .expect("Panic on upgrade_in_event_loop; cannot set model on ItemsViewAdapter");
}

async fn back(title: String, window_handle: Weak<ui::MainWindow>) {
    window_handle
        .upgrade_in_event_loop(move |main_window| {
            let current_page = main_window
                .global::<ui::ItemsViewAdapter>()
                .get_current_page();

            main_window
                .global::<ui::ItemsViewAdapter>()
                .set_current_page((current_page - 1).max(0));

            main_window
                .global::<ui::ItemsViewAdapter>()
                .set_title(title.into());
        })
        .expect("Panic on upgrade_in_event_loop; cannot set model on ItemsViewAdapter");

    // wait for page animation to finish FIXME read duration from animation
    std::thread::sleep(std::time::Duration::from_millis(500));

    window_handle
        .upgrade_in_event_loop(move |main_window| {
            let model = main_window.global::<ui::ItemsViewAdapter>().get_model();
            let current_count = model.row_count();
            if let Some(model) = model
                .as_any()
                .downcast_ref::<VecModel<ModelRc<ui::ListViewItem>>>()
            {
                model.remove(current_count - 1);
            }
        })
        .expect("Panic on upgrade_in_event_loop; cannot set model on ItemsViewAdapter");
}

fn item_type_to_icon(window_handle: Weak<ui::MainWindow>, item_type: model::ItemType) -> Image {
    if let Some(main_window) = window_handle.upgrade() {
        return match item_type {
            ItemType::Folder => main_window.global::<ui::Icons>().get_folder(),
            ItemType::Text => main_window.global::<ui::Icons>().get_description(),
            // FIXEME: image icon
            // ItemType::Image => main_window.global::<ui::Icons>().get_description(),
            ItemType::Unknown => Image::default(),
        };
    }

    Image::default()
}
