// SPDX-FileCopyrightText: 2023 Florian Blasius <co_sl@tutanota.com>
// SPDX-License-Identifier: GPL-3.0-only

use crate::model::{FileModel, FileType};
use crate::ui;
use slint::{ComponentHandle, Image, Model, ModelExt, ModelRc, VecModel, Weak};

use std::rc::Rc;

pub struct FilesProxyModel {
    root: FileModel,
    source: Rc<VecModel<FileModel>>,
    wrapped_model: ModelRc<FileModel>,
    window_handle: Weak<ui::MainWindow>,
}

impl FilesProxyModel {
    pub fn new(
        root: FileModel,
        files: Vec<FileModel>,
        window_handle: Weak<ui::MainWindow>,
    ) -> Self {
        let source = Rc::new(VecModel::from(files));

        Self {
            root,
            source: source.clone(),
            wrapped_model: source.into(),
            window_handle,
        }
    }

    pub fn get(&self, index: usize) -> Option<FileModel> {
        self.wrapped_model.row_data(index)
    }

    pub fn set(&self, index: usize, new_file_model: FileModel) {
        self.wrapped_model.set_row_data(index, new_file_model);
    }

    pub fn remove_item(&self, file_model: FileModel) -> Option<FileModel> {
        for i in 0..self.source.row_count() {
            if let Some(source_file_model) = self.source.row_data(i) {
                if !source_file_model.eq(&file_model) {
                    continue;
                }
            }

            return Some(self.source.remove(i));
        }

        None
    }

    pub fn root(&self) -> &FileModel {
        &self.root
    }

    pub fn push(&self, file_model: FileModel) {
        self.source.push(file_model);
    }

    pub fn as_sort_by<F>(&self, sort_function: F) -> FilesProxyModel
    where
        F: FnMut(&FileModel, &FileModel) -> core::cmp::Ordering + 'static,
    {
        Self {
            root: self.root.clone(),
            source: self.source.clone(),
            wrapped_model: Rc::new(self.wrapped_model.clone().sort_by(sort_function)).into(),
            window_handle: self.window_handle.clone(),
        }
    }

    pub fn as_filter_by<F>(&self, filter_function: F) -> FilesProxyModel
    where
        F: Fn(&FileModel) -> bool + 'static,
    {
        Self {
            root: self.root.clone(),
            source: self.source.clone(),
            wrapped_model: Rc::new(self.wrapped_model.clone().filter(filter_function)).into(),
            window_handle: self.window_handle.clone(),
        }
    }
}

impl Model for FilesProxyModel {
    type Data = ui::ListViewItem;

    fn row_count(&self) -> usize {
        self.wrapped_model.row_count()
    }

    fn row_data(&self, row: usize) -> Option<Self::Data> {
        if let Some(row_data) = self.wrapped_model.row_data(row) {
            return Some(ui::ListViewItem {
                leading_icon: item_type_to_icon(&self.window_handle, row_data.file_type()),
                text: row_data.name().unwrap_or_default().into(),
                highlighted: row_data.is_dir(),
                ..Default::default()
            });
        }

        None
    }

    fn model_tracker(&self) -> &dyn slint::ModelTracker {
        self.wrapped_model.model_tracker()
    }

    fn as_any(&self) -> &dyn core::any::Any {
        self
    }
}

fn item_type_to_icon(window_handle: &Weak<ui::MainWindow>, item_type: FileType) -> Image {
    if let Some(main_window) = window_handle.upgrade() {
        return match item_type {
            FileType::Dir => main_window.global::<ui::Icons>().get_folder(),
            FileType::Text => main_window.global::<ui::Icons>().get_description(),
            FileType::Image => main_window.global::<ui::Icons>().get_image(),
            FileType::Unknown => main_window.global::<ui::Icons>().get_insert_drive_file(),
        };
    }

    Image::default()
}
