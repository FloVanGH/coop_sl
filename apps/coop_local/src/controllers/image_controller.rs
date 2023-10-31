// SPDX-FileCopyrightText: 2023 Florian Blasius <co_sl@tutanota.com>
// SPDX-License-Identifier: GPL-3.0-only

use std::{
    cell::{Cell, RefCell},
    collections::HashMap,
    rc::Rc,
};

use crate::{
    models::{FileModel, FileType},
    repositories::{FilesRepository, ImageRepository},
    ui::*,
    Callback,
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
    image_repository: ImageRepository,
    current_item: Rc<Cell<usize>>,
    model: Rc<RefCell<Vec<FileModel>>>,
    image_cache: Rc<RefCell<HashMap<FileModel, Image>>>,
    show_about_callback: Rc<Callback<(), ()>>,
    back_callback: Rc<Callback<(), ()>>,
}

impl ImageController {
    pub fn new(
        view_handle: Weak<MainWindow>,
        files_repository: FilesRepository,
        image_repository: ImageRepository,
    ) -> Self {
        let controller = Self {
            view_handle,
            files_repository,
            image_repository,

            current_item: Rc::new(Cell::new(0)),
            model: Rc::new(RefCell::new(vec![])),
            image_cache: Rc::new(RefCell::new(HashMap::new())),
            show_about_callback: Rc::new(Callback::default()),
            back_callback: Rc::new(Callback::default()),
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

                adapter.on_next({
                    let controller = controller.clone();
                    move || controller.next()
                });

                adapter.on_previous({
                    let controller = controller.clone();
                    move || controller.previous()
                });
            }
        });

        controller
    }

    pub fn load_image(&self, file_model: FileModel) {
        if file_model.file_type() != FileType::Image {
            return;
        }

        if let Some(parent_file_model) = file_model.parent().map(FileModel::new) {
            self.model.borrow_mut().clear();
            self.current_item.set(0);
            self.image_cache.borrow_mut().clear();

            upgrade_adapter(&self.view_handle, |adapter| {
                adapter.set_image(Image::default());
                adapter.set_title("".into());
                adapter.set_loading(true);
            });

            let view_handle = self.view_handle.clone();
            let file_model_clone = file_model.clone();
            let repository = self.image_repository.clone();
            let controller = self.clone();

            let _ = slint::spawn_local(async move {
                let rt = Builder::new_current_thread().enable_all().build().unwrap();
                let (tx, rx) = oneshot::channel();

                let _ = std::thread::spawn({
                    let file_model = file_model_clone.clone();
                    move || {
                        rt.block_on(async move {
                            let _ = tx
                                .send(repository.image_list(&parent_file_model, &file_model).await);
                        });
                    }
                });

                if let Ok(Ok(mut images)) = rx.await {
                    if images.is_empty() {
                        upgrade_adapter(&view_handle, move |adapter| {
                            adapter.set_loading(false);
                            adapter.invoke_back();
                        });

                        return;
                    }

                    let single_image = images.len() == 1;

                    upgrade_adapter(&view_handle, move |adapter| {
                        adapter.set_single_image(single_image);
                    });

                    controller.model.borrow_mut().append(&mut images);
                    controller.show_image(0);
                }
            });
        }
    }

    pub fn on_back(&self, callback: impl Fn() + 'static) {
        self.back_callback.on(move |&()| {
            callback();
        });
    }

    pub fn on_show_about(&self, mut callback: impl FnMut() + 'static) {
        self.show_about_callback.on(move |&()| {
            callback();
        });
    }

    pub fn invoke_back(&self) {
        self.back_callback.invoke(&());
    }

    fn get_context_menu(&self) -> ModelRc<ListViewItem> {
        let items = VecModel::default();

        if let Some(current_image) = self.model.borrow().get(self.current_item.get()) {
            if let Ok(readonly) = current_image.readonly() {
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

    fn show_image(&self, index: usize) {
        if let Some(file_model) = self.model.borrow().get(index).cloned() {
            if let Some(image) = self.image_cache.borrow().get(&file_model).cloned() {
                upgrade_adapter(&self.view_handle, move |adapter| {
                    adapter.set_image(image);
                    adapter.set_title(file_model.name().unwrap_or_default().into());
                    adapter.set_loading(false);
                });
            } else {
                upgrade_adapter(&self.view_handle, |adapter| {
                    adapter.set_loading(true);
                });

                let view_handle = self.view_handle.clone();
                let repository = self.image_repository.clone();
                let image_cache = self.image_cache.clone();

                let _ = slint::spawn_local(async move {
                    let rt = Builder::new_current_thread().enable_all().build().unwrap();
                    let (tx, rx) = oneshot::channel();

                    let _ = std::thread::spawn({
                        let file_model = file_model.clone();
                        move || {
                            rt.block_on(async move {
                                let _ = tx.send(repository.load_image(&file_model).await);
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
                        let image = Image::from_rgba8(buffer);
                        image_cache
                            .borrow_mut()
                            .insert(file_model.clone(), image.clone());

                        upgrade_adapter(&view_handle, move |adapter| {
                            adapter.set_image(image);
                            adapter.set_title(file_model.name().unwrap_or_default().into());
                            adapter.set_loading(false);
                        });
                    }
                });
            }
        }
    }

    fn execute_context_menu_action(&self, spec: &str) {
        match spec {
            context_menu::REMOVE => self.remove(),
            context_menu::ABOUT => {
                self.show_about_callback.invoke(&());
            }
            _ => {}
        }
    }

    fn remove(&self) {
        let current_index = self.current_item.get();
        let current_image = self.model.borrow().get(current_index).cloned();
        if let Some(current_image) = current_image {
            if self.files_repository.remove(&current_image) {
                self.model.borrow_mut().remove(current_index);
                self.image_cache.borrow_mut().remove(&current_image);
            }
        }

        if self.model.borrow().is_empty() {
            self.invoke_back();
        } else {
            self.previous();
        }
    }

    fn next(&self) {
        let mut next = self.current_item.get() + 1;

        if next >= self.model.borrow().len() {
            next = 0;
        }

        self.current_item.set(next);
        self.show_image(next);
    }

    fn previous(&self) {
        let mut previous = self.current_item.get() as i32 - 1;

        if previous < 0 {
            previous = self.model.borrow().len() as i32 - 1;
        }

        self.current_item.set(previous as usize);
        self.show_image(previous as usize);
    }
}

fn upgrade_adapter(view_handle: &Weak<MainWindow>, func: impl FnOnce(ImageAdapter) + 'static) {
    if let Some(view) = view_handle.upgrade() {
        func(view.global::<ImageAdapter>());
    }
}
