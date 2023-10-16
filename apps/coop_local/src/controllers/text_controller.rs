// SPDX-FileCopyrightText: 2023 Florian Blasius <co_sl@tutanota.com>
// SPDX-License-Identifier: GPL-3.0-only

use std::{
    cell::{Cell, RefCell},
    collections::HashMap,
    rc::Rc,
};

use crate::{
    models::{FileModel, FileType, TextModel},
    repositories::{FilesRepository, TextRepository},
    ui::*,
};
use slint::*;
use tokio::runtime::Builder;
use tokio::sync::oneshot;

mod context_menu {
    pub const SAVE: &str = "save";
    pub const REMOVE: &str = "remove";
    pub const ABOUT: &str = "about";
}

#[derive(Clone)]
pub struct TextController {
    view_handle: Weak<MainWindow>,
    files_repository: FilesRepository,
    text_repository: TextRepository,
    show_about_callback: Rc<RefCell<Box<dyn FnMut() + 'static>>>,
    current_item: Rc<Cell<usize>>,
    texts: Rc<RefCell<Vec<FileModel>>>,
    text_cache: Rc<RefCell<HashMap<FileModel, TextModel>>>,
}

impl TextController {
    pub fn new(
        view_handle: Weak<MainWindow>,
        files_repository: FilesRepository,
        text_repository: TextRepository,
    ) -> Self {
        let controller = Self {
            view_handle,
            files_repository,
            text_repository,
            show_about_callback: Rc::new(RefCell::new(Box::new(|| {}))),
            current_item: Rc::new(Cell::new(0)),
            texts: Rc::new(RefCell::new(vec![])),
            text_cache: Rc::new(RefCell::new(HashMap::new())),
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

                adapter.on_update_text({
                    let controller = controller.clone();
                    move |text| controller.update_text(text.as_str())
                });
            }
        });

        controller
    }

    pub fn load_text(&self, file_model: FileModel) {
        if file_model.file_type() != FileType::Text {
            return;
        }

        if let Some(parent_file_model) = file_model.parent().map(FileModel::new) {
            self.texts.borrow_mut().clear();
            self.current_item.set(0);
            self.text_cache.borrow_mut().clear();

            upgrade_adapter(&self.view_handle, |adapter| {
                adapter.set_text("".into());
                adapter.set_title("".into());
                adapter.set_loading(true);
            });

            let view_handle = self.view_handle.clone();
            let file_model_clone = file_model.clone();
            let repository = self.text_repository.clone();
            let controller = self.clone();

            let _ = slint::spawn_local(async move {
                let rt = Builder::new_current_thread().enable_all().build().unwrap();
                let (tx, rx) = oneshot::channel();

                let _ = std::thread::spawn({
                    let file_model = file_model_clone.clone();
                    move || {
                        rt.block_on(async move {
                            let _ = tx
                                .send(repository.text_list(&parent_file_model, &file_model).await);
                        });
                    }
                });

                if let Ok(Ok(mut texts)) = rx.await {
                    if texts.is_empty() {
                        upgrade_adapter(&view_handle, move |adapter| {
                            adapter.set_loading(false);
                            adapter.invoke_back();
                        });

                        return;
                    }

                    let single_text = texts.len() == 1;

                    upgrade_adapter(&view_handle, move |adapter| {
                        adapter.set_single_text(single_text);
                    });

                    upgrade_adapter(&view_handle, move |adapter| {
                        adapter.set_single_text(single_text);
                    });

                    controller.texts.borrow_mut().append(&mut texts);
                    controller.show_text(0);
                }
            });
        }
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

        if let Some(current_text) = self.texts.borrow().get(self.current_item.get()) {
            if let Ok(readonly) = current_text.readonly() {
                if !readonly {
                    if let Some(text) = self.text_cache.borrow().get(current_text) {
                        if text.has_changes() {
                            items.push(ListViewItem {
                                text: "Save".into(),
                                spec: context_menu::SAVE.into(),
                                ..Default::default()
                            });
                        }
                    }

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

    fn show_text(&self, index: usize) {
        if let Some(file_model) = self.texts.borrow().get(index).cloned() {
            if let Some(text) = self.text_cache.borrow().get(&file_model).cloned() {
                upgrade_adapter(&self.view_handle, move |adapter| {
                    adapter.set_text(text.to_string().into());
                    adapter.set_title(file_model.name().unwrap_or_default().into());
                    adapter.set_read_only(file_model.readonly().unwrap_or_default());
                    adapter.set_loading(false);
                    adapter.set_has_changes(text.has_changes());
                });
            } else {
                upgrade_adapter(&self.view_handle, |adapter| {
                    adapter.set_loading(true);
                });

                let view_handle = self.view_handle.clone();
                let repository = self.text_repository.clone();
                let text_cache = self.text_cache.clone();

                let _ = slint::spawn_local(async move {
                    let rt = Builder::new_current_thread().enable_all().build().unwrap();
                    let (tx, rx) = oneshot::channel();

                    let _ = std::thread::spawn({
                        let file_model = file_model.clone();
                        move || {
                            rt.block_on(async move {
                                let _ = tx.send(repository.load_text(&file_model).await);
                            });
                        }
                    });

                    if let Ok(Ok(text)) = rx.await {
                        text_cache
                            .borrow_mut()
                            .insert(file_model.clone(), text.clone());

                        upgrade_adapter(&view_handle, move |adapter| {
                            adapter.set_text(text.to_string().into());
                            adapter.set_title(file_model.name().unwrap_or_default().into());
                            adapter.set_read_only(file_model.readonly().unwrap_or_default());
                            adapter.set_loading(false);
                            adapter.set_has_changes(false);
                        });
                    }
                });
            }
        }
    }

    fn execute_context_menu_action(&self, spec: &str) {
        match spec {
            context_menu::REMOVE => self.remove(),
            context_menu::SAVE => self.save(),
            context_menu::ABOUT => {
                if let Ok(mut callback) = self.show_about_callback.try_borrow_mut() {
                    callback();
                }
            }
            _ => {}
        }
    }

    fn remove(&self) {
        let current_index = self.current_item.get();
        let current_text = self.texts.borrow().get(current_index).cloned();
        if let Some(current_text) = current_text {
            if self.files_repository.remove(&current_text) {
                self.texts.borrow_mut().remove(current_index);
                self.text_cache.borrow_mut().remove(&current_text);
            }
        }

        if self.texts.borrow().is_empty() {
            upgrade_adapter(&self.view_handle, move |adapter| {
                adapter.invoke_back();
            });
        } else {
            self.previous();
        }
    }

    fn next(&self) {
        let mut next = self.current_item.get() + 1;

        if next >= self.texts.borrow().len() {
            next = 0;
        }

        self.current_item.set(next);
        self.show_text(next);
    }

    fn previous(&self) {
        let mut previous = self.current_item.get() as i32 - 1;

        if previous < 0 {
            previous = self.texts.borrow().len() as i32 - 1;
        }

        self.current_item.set(previous as usize);
        self.show_text(previous as usize);
    }

    fn update_text(&self, text_update: &str) {
        let mut has_changes = false;

        let file_model = self.texts.borrow().get(self.current_item.get()).cloned();
        if let Some(file_model) = file_model {
            if let Some(text) = self.text_cache.borrow_mut().get_mut(&file_model) {
                text.set_text_update(text_update);
                has_changes = text.has_changes();
            }
        }

        upgrade_adapter(&self.view_handle, move |adapter| {
            adapter.set_has_changes(has_changes);
        })
    }

    fn save(&self) {
        let file_model = self.texts.borrow().get(self.current_item.get()).cloned();

        if let Some(file_model) = file_model {
            let text_model = self.text_cache.borrow().get(&file_model).cloned();
            let text_cache = self.text_cache.clone();
            let view_handle = self.view_handle.clone();

            if let Some(text_model) = text_model {
                upgrade_adapter(&self.view_handle, |adapter| {
                    adapter.set_loading(true);
                });

                let repository = self.text_repository.clone();

                let _ = slint::spawn_local(async move {
                    let rt = Builder::new_current_thread().enable_all().build().unwrap();
                    let (tx, rx) = oneshot::channel();

                    let _ = std::thread::spawn({
                        let file_model = file_model.clone();
                        move || {
                            rt.block_on(async move {
                                let _ = tx.send(repository.save(&file_model, &text_model).await);
                            });
                        }
                    });

                    if let Ok(Ok(success)) = rx.await {
                        upgrade_adapter(&view_handle, |adapter| {
                            adapter.set_loading(false);
                        });

                        if success {
                            if let Some(text) = text_cache.borrow_mut().get_mut(&file_model) {
                                text.update_text();

                                upgrade_adapter(&view_handle, |adapter| {
                                    adapter.set_has_changes(false);
                                });
                            }
                        }
                    }
                });
            }
        }
    }
}

fn upgrade_adapter(view_handle: &Weak<MainWindow>, func: impl FnOnce(TextAdapter) + 'static) {
    if let Some(view) = view_handle.upgrade() {
        func(view.global::<TextAdapter>());
    }
}
