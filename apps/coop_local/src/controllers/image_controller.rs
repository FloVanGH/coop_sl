// SPDX-FileCopyrightText: 2023 Florian Blasius <co_sl@tutanota.com>
// SPDX-License-Identifier: GPL-3.0-only

use std::{cell::RefCell, rc::Rc};

use crate::{
    models::{FileModel, FileType},
    repositories::FilesRepository,
    ui::*,
};
use slint::*;
use tokio::runtime::Builder;
use tokio::sync::oneshot;

mod context_menu {
    pub const REMOVE: &str = "remove";
    pub const ABOUT: &str = "about";
}

#[derive(Clone)]
pub struct ImageController {
    view_handle: Weak<MainWindow>,
    files_repository: FilesRepository,
    file_model: Rc<RefCell<Option<FileModel>>>,
    show_about_callback: Rc<RefCell<Box<dyn FnMut() + 'static>>>,
}

impl ImageController {
    pub fn new(view_handle: Weak<MainWindow>, files_repository: FilesRepository) -> Self {
        let controller = Self {
            view_handle,
            files_repository,
            file_model: Rc::new(RefCell::new(None)),
            show_about_callback: Rc::new(RefCell::new(Box::new(|| {}))),
        };

        upgrade_adapter(&controller.view_handle, {
            let controller = controller.clone();

            // connect show context menu
            move |adapter| {
                adapter.on_get_context_menu({
                    let controller = controller.clone();
                    move || controller.get_context_menu()
                });

                adapter.on_context_menu_action({
                    let controller = controller.clone();
                    move |spec| controller.execute_context_menu_action(spec.as_str())
                });
            }
        });

        controller
    }

    pub fn show_image(&self, file_model: FileModel) {
        if file_model.file_type() != FileType::Image {
            return;
        }

        upgrade_adapter(&self.view_handle, |adapter| {
            adapter.set_image(Image::default());
            adapter.set_title("".into());
            adapter.set_loading(true);
        });

        let view_handle = self.view_handle.clone();
        let file_model_clone = file_model.clone();

        let _ = slint::spawn_local(async move {
            let rt = Builder::new_current_thread().enable_all().build().unwrap();
            let (tx, rx) = oneshot::channel();

            let _ = std::thread::spawn({
                let file_model = file_model_clone.clone();
                move || {
                    rt.block_on(async move {
                        let _ = tx.send(image::open(file_model.path()));
                    });
                }
            });

            if let Ok(Ok(image)) = rx.await {
                let image = image.into_rgba8();
                let buffer = SharedPixelBuffer::<Rgba8Pixel>::clone_from_slice(
                    image.as_raw(),
                    image.width(),
                    image.height(),
                );

                upgrade_adapter(&view_handle, move |adapter| {
                    adapter.set_image(Image::from_rgba8(buffer));
                    adapter.set_title(file_model_clone.name().unwrap_or_default().into());
                    adapter.set_loading(false);
                });
            }
        });

        *self.file_model.borrow_mut() = Some(file_model);
    }

    pub fn on_back(&self, func: impl FnMut() + 'static) {
        upgrade_adapter(&self.view_handle, move |adapter| {
            adapter.on_back(func);
        });
    }

    pub fn on_show_about(&self, callback: impl FnMut() + 'static) {
        *self.show_about_callback.borrow_mut() = Box::new(callback);
    }

    fn get_context_menu(&self) -> ModelRc<ListViewItem> {
        let items = VecModel::default();

        if let Some(file_model) = &*self.file_model.borrow() {
            if let Ok(readonly) = file_model.readonly() {
                if !readonly {
                    items.push(ListViewItem {
                        text: "Move to bin".into(),
                        spec: context_menu::REMOVE.into(),
                        ..Default::default()
                    });
                }
            }
        }

        items.push(ListViewItem {
            text: "About".into(),
            spec: context_menu::ABOUT.into(),
            ..Default::default()
        });

        Rc::new(items).into()
    }

    fn execute_context_menu_action(&self, spec: &str) {
        match spec {
            context_menu::REMOVE => self.remove(),
            context_menu::ABOUT => {
                if let Ok(mut callback) = self.show_about_callback.try_borrow_mut() {
                    callback();
                }
            }
            _ => {}
        }
    }

    fn remove(&self) {
        if let Some(file_model) = &*self.file_model.borrow() {
            self.files_repository.remove(file_model);
        }

        upgrade_adapter(&self.view_handle, move |adapter| {
            adapter.invoke_back();
        });

        *self.file_model.borrow_mut() = None;
    }
}

fn upgrade_adapter(view_handle: &Weak<MainWindow>, func: impl FnOnce(ImageAdapter) + 'static) {
    if let Some(view) = view_handle.upgrade() {
        func(view.global::<ImageAdapter>());
    }
}
