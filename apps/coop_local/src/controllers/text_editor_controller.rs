// SPDX-FileCopyrightText: 2023 Florian Blasius <co_sl@tutanota.com>
// SPDX-License-Identifier: GPL-3.0-only

use std::{rc::Rc, sync::Arc};

use crate::{
    models::{FileModel, FileType, TextEditorModel, TextModel},
    repositories::traits::TextEditorRepository,
    ui::*,
    Callback,
};
use slint::*;
use tokio::runtime::Builder;
use tokio::sync::oneshot;

mod context_menu {
    pub const SAVE: &str = "save";
    pub const REMOVE: &str = "remove";
    pub const ABOUT: &str = "about";
}

pub struct TextEditorController<T: TextEditorRepository> {
    repository: Arc<T>,
    model: Rc<TextEditorModel>,
    show_about_callback: Rc<Callback<(), ()>>,
    back_callback: Rc<Callback<(), ()>>,
    loading_changed_callback: Rc<Callback</* loading */ bool, ()>>,
    is_single_text_changed_callback: Rc<Callback</* is_single_text */ bool, ()>>,
    current_model_changed_callback: Rc<Callback<TextModel, ()>>,
}

impl<T: TextEditorRepository + Clone + Send + Sync + 'static> TextEditorController<T> {
    pub fn new(text_repository: T) -> Self {
        Self {
            repository: Arc::new(text_repository),
            model: Rc::new(TextEditorModel::default()),
            show_about_callback: Rc::new(Callback::default()),
            back_callback: Rc::new(Callback::default()),
            loading_changed_callback: Rc::new(Callback::default()),
            is_single_text_changed_callback: Rc::new(Callback::default()),
            current_model_changed_callback: Rc::new(Callback::default()),
        }
    }

    pub fn load(&self, file_model: FileModel, skip_clean: bool, load_async: bool) {
        if file_model.file_type() != FileType::Text {
            return;
        }

        if let Some(parent_file_model) = file_model.parent().map(FileModel::new) {
            self.model.clear();

            // clean the view
            if !skip_clean {
                self.current_model_changed_callback
                    .invoke(&TextModel::default());
            }

            self.loading_changed_callback.invoke(&true);

            let file_model_clone = file_model.clone();
            let repository = self.repository.clone();
            let loading_changed_callback = self.loading_changed_callback.clone();
            let current_model_changed_callback = self.current_model_changed_callback.clone();
            let is_single_text_changed_callback = self.is_single_text_changed_callback.clone();
            let back_callback = self.back_callback.clone();
            let model = self.model.clone();

            if load_async {
                let _ = slint::spawn_local(async move {
                    let rt = Builder::new_current_thread().enable_all().build().unwrap();
                    let (tx, rx) = oneshot::channel();

                    let _ = std::thread::spawn({
                        let file_model = file_model_clone.clone();
                        let repository = repository.clone();
                        move || {
                            rt.block_on(async move {
                                let _ =
                                    tx.send(repository.text_list(&parent_file_model, &file_model));
                            });
                        }
                    });

                    if let Ok(Ok(mut texts)) = rx.await {
                        if texts.is_empty() {
                            loading_changed_callback.invoke(&false);
                            back_callback.invoke(&());
                            return;
                        }

                        is_single_text_changed_callback.invoke(&(texts.len() == 1));
                        model.append(&mut texts);

                        show_text(
                            0,
                            repository,
                            model,
                            loading_changed_callback,
                            current_model_changed_callback,
                            true,
                        );
                    }
                });
            } else {
                let file_model = file_model_clone.clone();
                if let Ok(mut texts) = repository.text_list(&parent_file_model, &file_model) {
                    if texts.is_empty() {
                        loading_changed_callback.invoke(&false);
                        back_callback.invoke(&());
                        return;
                    }

                    is_single_text_changed_callback.invoke(&(texts.len() == 1));
                    model.append(&mut texts);

                    show_text(
                        0,
                        repository,
                        model,
                        loading_changed_callback,
                        current_model_changed_callback,
                        false,
                    );
                }
            }
        }
    }

    pub fn get_context_menu(&self) -> ModelRc<ListViewItem> {
        let items = VecModel::default();

        if let Some(current_text) = self.model.current_file() {
            if let Ok(readonly) = current_text.readonly() {
                if !readonly {
                    if let Some(text) = self.model.get_text(&current_text) {
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

    pub fn execute_context_menu_action(&self, spec: &str, load_async: bool) {
        match spec {
            context_menu::REMOVE => self.remove(load_async),
            context_menu::SAVE => self.save(load_async),
            context_menu::ABOUT => {
                self.show_about_callback.invoke(&());
            }
            _ => {}
        }
    }

    pub fn next(&self, load_async: bool) {
        let mut next = self.model.current_index() + 1;

        if next >= self.model.len() {
            next = 0;
        }

        self.model.set_current_index(next);
        show_text(
            next,
            self.repository.clone(),
            self.model.clone(),
            self.loading_changed_callback.clone(),
            self.current_model_changed_callback.clone(),
            load_async,
        );
    }

    pub fn previous(&self, load_async: bool) {
        let mut previous = self.model.current_index() as i32 - 1;

        if previous < 0 {
            previous = self.model.len() as i32 - 1;
        }

        self.model.set_current_index(previous as usize);

        show_text(
            previous as usize,
            self.repository.clone(),
            self.model.clone(),
            self.loading_changed_callback.clone(),
            self.current_model_changed_callback.clone(),
            load_async,
        );
    }

    pub fn update_text(&self, text_update: &str) -> bool {
        let file_model = self.model.current_file();
        if let Some(file_model) = file_model {
            if let Some(mut text) = self.model.get_text(&file_model) {
                text.set_text_update(text_update);
                let has_changes = text.has_changes();
                self.model.set_text(file_model, text);
                return has_changes;
            }
        }

        false
    }

    pub fn on_loading_changed(&self, callback: impl Fn(/* loading */ &bool) + 'static) {
        self.loading_changed_callback.on(callback);
    }

    pub fn on_single_text_changed(&self, callback: impl Fn(/* single_text */ &bool) + 'static) {
        self.is_single_text_changed_callback.on(callback);
    }

    pub fn on_current_model_changed(&self, callback: impl Fn(&TextModel) + 'static) {
        self.current_model_changed_callback.on(callback);
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

    fn remove(&self, load_async: bool) {
        let current_index = self.model.current_index();
        if let Some(current_file) = self.model.current_file() {
            if self.repository.remove(&current_file) {
                self.model.remove(current_index);
            }
        }

        if self.model.is_empty() {
            self.invoke_back();
        } else {
            self.previous(load_async);
        }
    }

    fn save(&self, load_async: bool) {
        if let Some(file_model) = self.model.current_file() {
            if let Some(mut text_model) = self.model.get_text(&file_model) {
                self.loading_changed_callback.invoke(&true);

                let repository = self.repository.clone();
                let loading_changed_callback = self.loading_changed_callback.clone();
                let current_model_changed_callback = self.current_model_changed_callback.clone();
                let model = self.model.clone();

                if load_async {
                    let _ = slint::spawn_local(async move {
                        let rt = Builder::new_current_thread().enable_all().build().unwrap();
                        let (tx, rx) = oneshot::channel();

                        let _ = std::thread::spawn({
                            let file_model = file_model.clone();
                            let text_model = text_model.clone();

                            move || {
                                rt.block_on(async move {
                                    let _ = tx.send(repository.save(&file_model, &text_model));
                                });
                            }
                        });

                        if let Ok(Ok(success)) = rx.await {
                            loading_changed_callback.invoke(&false);

                            if success {
                                text_model.update_text();

                                current_model_changed_callback.invoke(&text_model);
                                model.set_text(file_model, text_model);
                            }
                        }
                    });
                } else if let Ok(success) = repository.save(&file_model, &text_model) {
                    if success {
                        text_model.update_text();
                        current_model_changed_callback.invoke(&text_model);
                        model.set_text(file_model, text_model);
                    }
                }
            }
        }
    }
}

fn show_text<T: TextEditorRepository + Send + Sync + 'static>(
    index: usize,
    repository: Arc<T>,
    model: Rc<TextEditorModel>,
    loading_changed_callback: Rc<Callback</* loading */ bool, ()>>,
    current_model_changed_callback: Rc<Callback<TextModel, ()>>,
    load_async: bool,
) {
    if let Some(file_model) = model.get_file(index) {
        if let Some(text_model) = model.get_text(&file_model) {
            current_model_changed_callback.invoke(&text_model);
        } else {
            loading_changed_callback.invoke(&true);

            let model = model.clone();

            if load_async {
                let _ = slint::spawn_local(async move {
                    let rt = Builder::new_current_thread().enable_all().build().unwrap();
                    let (tx, rx) = oneshot::channel();

                    let _ = std::thread::spawn({
                        let file_model = file_model.clone();
                        move || {
                            rt.block_on(async move {
                                let _ = tx.send(repository.load_text(&file_model));
                            });
                        }
                    });

                    if let Ok(Ok(text_model)) = rx.await {
                        model.set_text(file_model.clone(), text_model.clone());
                        current_model_changed_callback.invoke(&text_model);
                    }
                });
            } else if let Ok(text_model) = repository.load_text(&file_model) {
                model.set_text(file_model.clone(), text_model.clone());
                current_model_changed_callback.invoke(&text_model);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::repositories::TextEditorRepositoryMock;
    use std::{cell::Cell, collections::HashMap};

    fn repository() -> TextEditorRepositoryMock {
        let file_one = FileModel::new("/text.txt");
        let file_two = FileModel::new("/text2.txt");
        let text_model_one = TextModel::new("File1.txt", "Hello world", false);
        let text_model_two = TextModel::new("File2.txt", "Hello world 2", true);

        let mut text_models = HashMap::new();
        text_models.insert(file_one.clone(), text_model_one.clone());
        text_models.insert(file_two.clone(), text_model_two.clone());

        TextEditorRepositoryMock::new(vec![file_one.clone(), file_two.clone()], text_models)
    }

    fn single_file_repository() -> TextEditorRepositoryMock {
        let file_one = FileModel::new("/text.txt");

        let text_model_one = TextModel::new("File1.txt", "Hello world", false);

        let mut text_models = HashMap::new();
        text_models.insert(file_one.clone(), text_model_one.clone());

        TextEditorRepositoryMock::new(vec![file_one.clone()], text_models)
    }

    #[test]
    fn test_load() {
        let repository = repository();
        let controller = TextEditorController::new(repository);

        controller.on_single_text_changed(|is_single_text| {
            assert!(!is_single_text);
        });
        controller.on_current_model_changed(|m| {
            assert_eq!(m, &TextModel::new("File1.txt", "Hello world", false));
        });
        controller.load(FileModel::new("/text.txt"), true, true);

        controller.on_current_model_changed(|m| {
            assert_eq!(m, &TextModel::new("File2.txt", "Hello world 2", false));
        });
        controller.load(FileModel::new("/text2.txt"), true, true);

        let repository = single_file_repository();
        let controller = TextEditorController::new(repository);

        controller.on_single_text_changed(|is_single_text| {
            assert!(is_single_text);
        });
        controller.on_current_model_changed(|m| {
            assert_eq!(m, &TextModel::new("File1.txt", "Hello world", false));
        });
        controller.load(FileModel::new("/text.txt"), true, true);
    }

    #[test]
    fn test_get_context_menu() {
        let repository = repository();
        let controller = TextEditorController::new(repository);
        controller.load(FileModel::new("/text.txt"), true, false);

        let context_menu_model = controller.get_context_menu();

        assert_eq!(
            context_menu_model.row_data(0).unwrap().spec,
            context_menu::REMOVE
        );
        assert_eq!(
            context_menu_model.row_data(1).unwrap().spec,
            context_menu::ABOUT
        );

        assert!(controller.update_text("text_update"));
        let context_menu_model = controller.get_context_menu();

        assert_eq!(
            context_menu_model.row_data(0).unwrap().spec,
            context_menu::SAVE
        );
        assert_eq!(
            context_menu_model.row_data(1).unwrap().spec,
            context_menu::REMOVE
        );
        assert_eq!(
            context_menu_model.row_data(2).unwrap().spec,
            context_menu::ABOUT
        );
    }

    #[test]
    fn test_execute_context_menu_action() {
        let repository = repository();
        let controller = TextEditorController::new(repository);
        controller.load(FileModel::new("/text.txt"), true, false);

        let check_about = Rc::new(Cell::new(false));
        controller.on_show_about({
            let check = check_about.clone();
            move || {
                check.set(true);
            }
        });

        controller.execute_context_menu_action(context_menu::ABOUT, false);
        assert!(check_about.get());

        assert!(controller.update_text("text_update"));
        controller.on_current_model_changed(|model| {
            assert_eq!(model, &TextModel::new("File1.txt", "text_update", false));
        });
        controller.execute_context_menu_action(context_menu::SAVE, false);

        // reset to prevent additional checks
        controller.on_current_model_changed(|_| {});

        let check_back = Rc::new(Cell::new(false));
        controller.on_back({
            let check_back = check_back.clone();
            move || {
                check_back.set(true);
            }
        });

        controller.remove(false);
        controller.remove(false);

        assert!(check_back.get());
    }

    #[test]
    fn test_next() {
        let repository = repository();
        let controller = TextEditorController::new(repository);
        controller.load(FileModel::new("/text.txt"), true, false);

        controller.on_current_model_changed(|model| {
            assert_eq!(model, &TextModel::new("File2.txt", "Hello world 2", true));
        });
        controller.next(false);

        controller.on_current_model_changed(|model| {
            assert_eq!(model, &TextModel::new("File1.txt", "Hello world", false));
        });
        controller.next(false);
    }

    #[test]
    fn test_previous() {
        let repository = repository();
        let controller = TextEditorController::new(repository);
        controller.load(FileModel::new("/text.txt"), true, false);

        controller.on_current_model_changed(|model| {
            assert_eq!(model, &TextModel::new("File2.txt", "Hello world 2", true));
        });
        controller.previous(false);

        controller.on_current_model_changed(|model| {
            assert_eq!(model, &TextModel::new("File1.txt", "Hello world", false));
        });
        controller.previous(false);
    }
}
