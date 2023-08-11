// SPDX-FileCopyrightText: 2023 Florian Blasius <co_sl@tutanota.com>
// SPDX-License-Identifier: GPL-3.0-only

use super::{FilesController, FilesMessage};
use crate::{
    model::{FileModel, FileType},
    repository, ui,
};
use slint::{ComponentHandle, ModelRc, VecModel};
use std::rc::Rc;

const OPEN: &str = "open";
const RENAME: &str = "rename";
const REMOVE: &str = "remove";
const ADD_TO_CLIPBOARD: &str = "add-to-clipboard";
const ADD_BOOKMARK: &str = "add-to-favorites";
const PASTE: &str = "paste";
const CLEAR_CLIPBOARD: &str = "clear-clipboard";
const NEW_FOLDER: &str = "new-folder";
const ABOUT: &str = "about";

pub fn on_main_menu_action(controller: FilesController, main_window: &ui::MainWindow) {
    main_window
        .global::<ui::FilesAdapter>()
        .on_main_menu_action(move |page_index, action| match action.as_str() {
            NEW_FOLDER => {
                if let Some(root) = controller.get_root(page_index as usize) {
                    controller.spawn_message(FilesMessage::CreateNewFolder {
                        page_index: page_index as usize,
                        root,
                    });
                }
            }
            CLEAR_CLIPBOARD => {
                controller.spawn_message(FilesMessage::ClearClipboard);
            }
            PASTE => {
                if let Some(root) = controller.get_root(page_index as usize) {
                    controller.spawn_message(FilesMessage::Paste {
                        page_index: page_index as usize,
                        root,
                    });
                }
            }

            ABOUT => controller.spawn_message(FilesMessage::ShowAboutDialog),
            _ => {}
        });
}

pub fn get_main_menu<R>(repository: R) -> ModelRc<ui::ListViewItem>
where
    R: repository::traits::FileRepository + std::marker::Send + Clone + 'static,
{
    let items = VecModel::default();

    // FIXME: only on writable roots
    items.push(ui::ListViewItem {
        text: "New folder".into(),
        spec: NEW_FOLDER.into(),
        ..Default::default()
    });

    if repository.can_paste() {
        items.push(ui::ListViewItem {
            text: "Paste".into(),
            spec: PASTE.into(),
            ..Default::default()
        });
        items.push(ui::ListViewItem {
            text: "Clear clipboard".into(),
            spec: CLEAR_CLIPBOARD.into(),
            ..Default::default()
        });
    }

    items.push(ui::ListViewItem {
        text: "About".into(),
        spec: ABOUT.into(),
        ..Default::default()
    });

    Rc::new(items).into()
}

pub fn on_context_menu_action(controller: FilesController, main_window: &ui::MainWindow) {
    main_window
        .global::<ui::FilesAdapter>()
        .on_context_menu_action(
            move |page_index, file_index, action| match action.as_str() {
                OPEN => {
                    if let Some(file_model) =
                        controller.get_file_model(page_index as usize, file_index as usize)
                    {
                        controller.spawn_message(FilesMessage::OpenExternal { file_model });
                    }
                }
                RENAME => {
                    if let Some(file_model) =
                        controller.get_file_model(page_index as usize, file_index as usize)
                    {
                        controller.spawn_message(FilesMessage::Rename {
                            page_index: page_index as usize,
                            file_index: file_index as usize,
                            file_model,
                        });
                    }
                }
                REMOVE => {
                    if let Some(file_model) =
                        controller.get_file_model(page_index as usize, file_index as usize)
                    {
                        controller.spawn_message(FilesMessage::Remove {
                            page_index: page_index as usize,
                            file_model,
                        });
                    }
                }
                ADD_TO_CLIPBOARD => {
                    if let Some(file_model) =
                        controller.get_file_model(page_index as usize, file_index as usize)
                    {
                        controller.spawn_message(FilesMessage::AddToClipboard { file_model });
                    }
                }
                ADD_BOOKMARK => controller.spawn_message(FilesMessage::AddBookmark {
                    page_index: page_index as usize,
                    file_index: file_index as usize,
                }),
                _ => {}
            },
        );
}

pub fn get_context_menu(file: &FileModel) -> ModelRc<ui::ListViewItem> {
    let items = VecModel::default();

    items.push(ui::ListViewItem {
        text: "Open".into(),
        spec: OPEN.into(),
        ..Default::default()
    });

    if let Ok(readonly) = file.readonly() {
        if !readonly {
            items.push(ui::ListViewItem {
                text: "Rename".into(),
                spec: RENAME.into(),
                ..Default::default()
            });

            items.push(ui::ListViewItem {
                text: "Move to bin".into(),
                spec: REMOVE.into(),
                ..Default::default()
            });
        }
    }
    items.push(ui::ListViewItem {
        text: "Add to clipboard".into(),
        spec: ADD_TO_CLIPBOARD.into(),
        ..Default::default()
    });

    if file.file_type().eq(&FileType::Dir) {
        items.push(ui::ListViewItem {
            text: "Add bookmark".into(),
            spec: ADD_BOOKMARK.into(),
            ..Default::default()
        });
    }

    Rc::new(items).into()
}
