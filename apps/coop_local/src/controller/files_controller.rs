// SPDX-FileCopyrightText: 2023 Florian Blasius <co_sl@tutanota.com>
// SPDX-License-Identifier: GPL-3.0-only

use super::{DialogResponse, DialogViewController, DialogViewMessage};
use crate::model::{FileModel, FileType};
use crate::proxy_models::FilesProxyModel;
use crate::{proxy_models, repository, ui};
use slint::*;
use std::io::Read;
use std::rc::Rc;
use std::{fs, io};
use tokio::runtime::Builder;
use tokio::sync::{mpsc, oneshot};

mod context_menu;

#[derive(Clone)]
pub struct FilesController {
    window_handle: Weak<ui::MainWindow>,
    spawn: mpsc::Sender<FilesMessage>,
}

impl FilesController {
    pub fn new<R>(
        main_window: &ui::MainWindow,
        repository: R,
        dialog_controller: DialogViewController,
    ) -> io::Result<Self>
    where
        R: repository::traits::FileRepository + Clone + std::marker::Send + 'static,
    {
        let rt = Builder::new_current_thread().enable_all().build()?;
        let (send, mut recv) = mpsc::channel(16);

        let controller = Self {
            window_handle: main_window.as_weak(),
            spawn: send,
        };

        init(&controller, &repository, main_window);

        std::thread::spawn({
            let window_handle = main_window.as_weak();

            move || {
                rt.block_on(async move {
                    while let Some(message) = recv.recv().await {
                        tokio::spawn(handle_message(
                            message,
                            repository.clone(),
                            dialog_controller.clone(),
                            window_handle.clone(),
                        ));
                    }
                });
            }
        });

        Ok(controller)
    }

    pub fn get_file_model(&self, page_index: usize, file_index: usize) -> Option<FileModel> {
        if let Some(main_window) = self.window_handle.upgrade() {
            if let Some(files) = main_window
                .global::<ui::FilesAdapter>()
                .get_files()
                .row_data(page_index)
            {
                if let Some(files_proxy) = files
                    .as_any()
                    .downcast_ref::<proxy_models::FilesProxyModel>()
                {
                    return files_proxy.row_data_as_file_model(file_index);
                }
            }
        }

        None
    }

    fn get_root(&self, page_index: usize) -> Option<FileModel> {
        if let Some(main_window) = self.window_handle.upgrade() {
            if let Some(files) = main_window
                .global::<ui::FilesAdapter>()
                .get_files()
                .row_data(page_index)
            {
                if let Some(files_proxy) = files
                    .as_any()
                    .downcast_ref::<proxy_models::FilesProxyModel>()
                {
                    return Some(files_proxy.root().clone());
                }
            }
        }

        None
    }

    pub fn spawn_message(&self, message: FilesMessage) {
        let _ = self.spawn.blocking_send(message);
    }
}

// setup

fn init<R>(controller: &FilesController, repository: &R, main_window: &ui::MainWindow)
where
    R: repository::traits::FileRepository + Clone + std::marker::Send + 'static,
{
    let adapter: ui::FilesAdapter<'_> = main_window.global::<ui::FilesAdapter>();

    adapter.on_open({
        let controller = controller.clone();
        move |page, file| {
            if let Some(file) = controller.get_file_model(page as usize, file as usize) {
                controller.spawn_message(FilesMessage::Open { file });
            }
        }
    });

    adapter.on_previous_page({
        let controller = controller.clone();
        move || {
            controller.spawn_message(FilesMessage::PreviousPage);
        }
    });

    adapter.set_files(Rc::new(VecModel::default()).into());

    adapter.on_get_main_menu({
        let repository = repository.clone();

        move || context_menu::get_main_menu(repository.clone())
    });

    adapter.on_get_file_context_menu({
        let controller = controller.clone();

        move |page_index, file_index: i32| {
            if let Some(file_model) =
                controller.get_file_model(page_index as usize, file_index as usize)
            {
                return context_menu::get_context_menu(&file_model);
            }

            VecModel::<ui::ListViewItem>::from_slice(&[])
        }
    });

    adapter.on_show({
        let controller = controller.clone();

        move |path| controller.spawn_message(FilesMessage::Load { path: path.into() })
    });

    context_menu::on_main_menu_action(controller.clone(), main_window);
    context_menu::on_context_menu_action(controller.clone(), main_window);
}

pub enum FilesMessage {
    // global operations
    Load {
        path: String,
    },
    PreviousPage,
    ShowAboutDialog,

    // file based operation
    Open {
        file: FileModel,
    },
    OpenExternal {
        file_model: FileModel,
    },
    Rename {
        page_index: usize,
        file_index: usize,
        file_model: FileModel,
    },
    Remove {
        page_index: usize,
        file_model: FileModel,
    },
    AddToClipboard {
        file_model: FileModel,
    },
    Paste {
        page_index: usize,
        root: FileModel,
    },
    ClearClipboard,
    CreateNewFolder {
        page_index: usize,
        root: FileModel,
    },
    AddBookmark {
        page_index: usize,
        file_index: usize,
    },
    CheckForModifications,
}

async fn handle_message<R>(
    message: FilesMessage,
    repository: R,
    dialog_controller: DialogViewController,
    window_handle: Weak<ui::MainWindow>,
) where
    R: repository::traits::FileRepository + std::marker::Send + Clone + 'static,
{
    match message {
        FilesMessage::Load { path } => {
            tokio::spawn(load(FileModel::new(path), window_handle, repository));
        }
        FilesMessage::Open { file } => {
            tokio::spawn(open(file, window_handle, repository));
        }
        FilesMessage::OpenExternal { file_model } => {
            tokio::spawn(open_external(file_model, repository));
        }
        FilesMessage::PreviousPage => {
            tokio::spawn(previous_page(window_handle));
        }
        FilesMessage::Rename {
            file_model,
            page_index,
            file_index,
        } => {
            tokio::spawn(rename_file(
                page_index,
                file_index,
                file_model,
                repository,
                dialog_controller,
                window_handle,
            ));
        }
        FilesMessage::Remove {
            page_index,
            file_model,
        } => {
            tokio::spawn(remove(page_index, file_model, window_handle, repository));
        }
        FilesMessage::AddToClipboard { file_model } => {
            tokio::spawn(add_to_clipboard(file_model, repository));
        }

        FilesMessage::AddBookmark {
            page_index,
            file_index,
        } => {
            tokio::spawn(add_bookmark(page_index, file_index, window_handle));
        }
        FilesMessage::Paste { page_index, root } => {
            tokio::spawn(paste(page_index, root, window_handle, repository));
        }
        FilesMessage::ClearClipboard => {
            tokio::spawn(clear_clipboard(repository));
        }
        FilesMessage::ShowAboutDialog => {
            tokio::spawn(show_about_dialog(dialog_controller));
        }
        FilesMessage::CreateNewFolder { page_index, root } => {
            tokio::spawn(create_new_folder(
                page_index,
                root,
                repository,
                dialog_controller,
                window_handle,
            ));
        }
        FilesMessage::CheckForModifications => {
            tokio::spawn(check_for_modifications(repository, window_handle));
        }
    };
}

async fn load<R>(root: FileModel, window_handle: Weak<ui::MainWindow>, repository: R)
where
    R: repository::traits::FileRepository + std::marker::Send + 'static,
{
    // don't open same page twice
    let window_handle_clone = window_handle.clone();
    if let Some(current_root) = current_root_async(window_handle_clone).await {
        if root.eq(&current_root) {
            return;
        }
    }

    if let Ok(files) = repository.files(&root) {
        upgrade_adapter(window_handle.clone(), move |adapter| {
            let adapter_files = adapter.get_files();
            let current_page = adapter.get_current_page();
            let number_of_pages = adapter_files.row_count() as i32;

            let title = root.name().unwrap_or_default().to_string();
            let files_view_model = proxy_models::FilesProxyModel::new(root, files, window_handle)
                .as_sort_by(|l, r| {
                    if l.file_type() == r.file_type() {
                        l.name().cmp(&r.name())
                    } else {
                        l.file_type().cmp(&r.file_type())
                    }
                })
                .as_filter_by(|f| !f.hidden());

            if let Some(files) = adapter_files
                .as_any()
                .downcast_ref::<VecModel<ModelRc<ui::ListViewItem>>>()
            {
                let new_page = ModelRc::new(files_view_model);
                if number_of_pages - 1 > current_page {
                    files.set_row_data(number_of_pages as usize - 1, new_page);
                } else {
                    files.push(new_page);
                }
            }

            let current_count = adapter_files.row_count();
            adapter.set_current_page(current_count as i32 - 1);
            adapter.set_title(title.into());
        });
    }
}

async fn previous_page(window_handle: Weak<ui::MainWindow>) {
    upgrade_adapter(window_handle.clone(), move |adapter| {
        if adapter.get_text_view_visible() {
            adapter.set_text_view_visible(false);
            let current_page = adapter.get_current_page();
            if let Some(root) = get_root(current_page as usize, &adapter) {
                adapter.set_title(root.name().unwrap_or_default().into());
            }

            return;
        }

        let current_page = adapter.get_current_page();
        let number_of_pages = adapter.get_files().row_count() as i32;

        if current_page + 2 == number_of_pages {
            if let Some(files) = adapter
                .get_files()
                .as_any()
                .downcast_ref::<VecModel<ModelRc<ui::ListViewItem>>>()
            {
                files.remove(number_of_pages as usize - 1);
            }
        }

        let new_current_page = (current_page - 1).max(0);

        adapter.set_current_page(new_current_page);

        if let Some(root) = get_root(new_current_page as usize, &adapter) {
            adapter.set_title(root.name().unwrap_or_default().into());
        }
    });
}

// handlers

async fn open<R>(file: FileModel, window_handle: Weak<ui::MainWindow>, repository: R)
where
    R: repository::traits::FileRepository + std::marker::Send + 'static,
{
    match file.file_type() {
        FileType::Dir => {
            tokio::spawn(load(file, window_handle, repository));
        }
        FileType::Text => {
            tokio::spawn(open_text_view(file, window_handle));
        }
        _ => {}
    }
}

async fn open_external<R>(file: FileModel, repository: R)
where
    R: repository::traits::FileRepository + std::marker::Send + 'static,
{
    let _ = repository.open(file);
}

async fn rename_file<R>(
    page_index: usize,
    file_index: usize,
    file_model: FileModel,
    repository: R,
    dialog_controller: DialogViewController,
    window_handle: Weak<ui::MainWindow>,
) where
    R: repository::traits::FileRepository + std::marker::Send + 'static,
{
    let (sender, mut receiver) = mpsc::channel(16);
    let old_name = file_model.name().unwrap_or_default().to_string();

    dialog_controller
        .spawn_message(DialogViewMessage::ShowTextInput {
            title: "Rename file".into(),
            default_button_text: "Rename".into(),
            text: old_name.clone(),
            respond_to: sender,
        })
        .await;

    if let Some(message) = receiver.recv().await {
        match message {
            DialogResponse::TextUpdate(new_name) => {
                if new_name.eq(&old_name) {
                    return;
                }

                if let Ok(updated_file) = repository.rename(file_model, new_name) {
                    upgrade_proxy_model(window_handle, page_index, move |proxy_model| {
                        proxy_model.set_row_data_as_file_model(file_index, updated_file);
                    });
                }
            }
            // do nothing
            DialogResponse::Canceled => {}
        }
    }
}

async fn remove<R>(
    page_index: usize,
    file_model: FileModel,
    window_handle: Weak<ui::MainWindow>,
    repository: R,
) where
    R: repository::traits::FileRepository + std::marker::Send + Clone + 'static,
{
    if repository.remove(&file_model) {
        upgrade_proxy_model(window_handle, page_index, move |proxy_model| {
            proxy_model.remove_from_source(file_model);
        });
    }
}

async fn add_to_clipboard<R>(file_model: FileModel, repository: R)
where
    R: repository::traits::FileRepository + std::marker::Send + Clone + 'static,
{
    repository.add_to_clipboard(file_model);
}

async fn paste<R>(
    page_index: usize,
    root: FileModel,
    window_handle: Weak<ui::MainWindow>,
    repository: R,
) where
    R: repository::traits::FileRepository + std::marker::Send + Clone + 'static,
{
    if !repository.can_paste() {
        return;
    }

    if let Ok(added_files) = repository.paste(&root) {
        upgrade_proxy_model(window_handle, page_index, move |proxy_model| {
            for file in added_files {
                proxy_model.push_to_source(file);
            }
        })
    };

    repository.clear_clipboard();
}

async fn show_about_dialog(dialog_controller: DialogViewController) {
    dialog_controller
        .spawn_message(DialogViewMessage::ShowAbout)
        .await;
}

async fn clear_clipboard<R>(repository: R)
where
    R: repository::traits::FileRepository + std::marker::Send + Clone + 'static,
{
    repository.clear_clipboard();
}

async fn create_new_folder<R>(
    page_index: usize,
    root: FileModel,
    repository: R,
    dialog_controller: DialogViewController,
    window_handle: Weak<ui::MainWindow>,
) where
    R: repository::traits::FileRepository + std::marker::Send + 'static,
{
    let (sender, mut receiver) = mpsc::channel(16);

    dialog_controller
        .spawn_message(DialogViewMessage::ShowTextInput {
            title: "Create new folder".into(),
            default_button_text: "Create".into(),
            text: "".into(),
            respond_to: sender,
        })
        .await;

    if let Some(message) = receiver.recv().await {
        match message {
            DialogResponse::TextUpdate(folder_name) => {
                if folder_name.is_empty() {
                    return;
                }

                if let Ok(new_folder) = repository.create_new_folder(&root, folder_name) {
                    upgrade_proxy_model(window_handle, page_index, move |proxy_model| {
                        proxy_model.push_to_source(new_folder);
                    });
                }
            }
            // do nothing
            DialogResponse::Canceled => {}
        }
    }
}

async fn check_for_modifications<R>(repository: R, window_handle: Weak<ui::MainWindow>)
where
    R: repository::traits::FileRepository + std::marker::Send + 'static,
{
    let window_handle_share = window_handle.clone();
    if let Some(root) = current_root_async(window_handle_share).await {
        if let Ok(mut files) = repository.files(&root) {
            upgrade_adapter(window_handle, move |adapter| {
                let current_page: usize = adapter.get_current_page() as usize;
                if let Some(model) = adapter.get_files().row_data(current_page) {
                    if let Some(proxy_model) = model
                        .as_any()
                        .downcast_ref::<proxy_models::FilesProxyModel>()
                    {
                        // if !root.modified() {
                        //     return;
                        // }

                        // proxy_model.set_root(root.clone());

                        if !files.is_empty() {
                            for row in 0..proxy_model.row_count_source() {
                                if let Some(file_model) = proxy_model.row_data_from_source(row) {
                                    if files.contains(&file_model) {
                                        files.remove(
                                            files.iter().position(|f| f.eq(&file_model)).unwrap(),
                                        );
                                        continue;
                                    }

                                    // file is no longer in the real directory but still in the ui (remove it)
                                    proxy_model.remove_from_source(file_model);
                                }
                            }

                            // add new files to proxy
                            for file in files {
                                proxy_model.push_to_source(file);
                            }
                            return;
                        }

                        // clears the proxy if there are no more files left
                        proxy_model.clear();
                    }
                }
            });
        }
    }
}

async fn add_bookmark(page_index: usize, file_index: usize, window_handle: Weak<ui::MainWindow>) {
    let _ = window_handle.upgrade_in_event_loop(move |main_window| {
        if let Some(files) = main_window
            .global::<ui::FilesAdapter>()
            .get_files()
            .row_data(page_index)
        {
            if let Some(files_proxy) = files
                .as_any()
                .downcast_ref::<proxy_models::FilesProxyModel>()
            {
                if let Some(file_model) = files_proxy.row_data_as_file_model(file_index) {
                    main_window
                        .global::<ui::SideBarAdapter>()
                        .invoke_add_bookmark(file_model.path().into());
                }
            }
        }
    });
}

// helper

async fn current_root_async(window_handle: Weak<ui::MainWindow>) -> Option<FileModel> {
    let (tx, rx) = oneshot::channel();

    upgrade_adapter(window_handle, |adapter| {
        let current_page = adapter.get_current_page();

        let _ = tx.send(get_root(current_page as usize, &adapter));
    });

    if let Ok(root) = rx.await {
        return root;
    }

    None
}

async fn open_text_view(file_model: FileModel, window_handle: Weak<ui::MainWindow>) {
    if let Ok(mut file) = fs::File::open(file_model.as_path()) {
        let mut text = String::default();
        if file.read_to_string(&mut text).is_ok() {
            upgrade_adapter(window_handle, move |adapter| {
                adapter.set_text_view_text(text.into());
                adapter.set_text_view_visible(true);
                adapter.set_title(file_model.name().unwrap_or_default().into());
            });
        }
    }
}

fn upgrade_adapter(
    window_handle: Weak<ui::MainWindow>,
    func: impl FnOnce(ui::FilesAdapter) + Send + 'static,
) {
    let _ = window_handle
        .upgrade_in_event_loop(move |main_window| func(main_window.global::<ui::FilesAdapter>()));
}

fn upgrade_proxy_model(
    window_handle: Weak<ui::MainWindow>,
    page_index: usize,
    func: impl FnOnce(&FilesProxyModel) + Send + 'static,
) {
    upgrade_adapter(window_handle, move |adapter| {
        if let Some(model) = adapter.get_files().row_data(page_index) {
            if let Some(proxy_model) = model
                .as_any()
                .downcast_ref::<proxy_models::FilesProxyModel>()
            {
                func(proxy_model);
            }
        }
    });
}

fn get_root(page_index: usize, adapter: &ui::FilesAdapter) -> Option<FileModel> {
    if let Some(files) = adapter.get_files().row_data(page_index) {
        if let Some(files_proxy) = files
            .as_any()
            .downcast_ref::<proxy_models::FilesProxyModel>()
        {
            return Some(files_proxy.root().clone());
        }
    }

    None
}
