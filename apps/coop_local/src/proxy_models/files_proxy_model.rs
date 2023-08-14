// SPDX-FileCopyrightText: 2023 Florian Blasius <co_sl@tutanota.com>
// SPDX-License-Identifier: GPL-3.0-only

use crate::model::{FileModel, FileType};
use crate::ui;
use slint::{ComponentHandle, Image, Model, ModelExt, ModelRc, VecModel, Weak};

use std::cell::RefCell;
use std::rc::Rc;

pub struct FilesProxyModel {
    root: Rc<RefCell<FileModel>>,
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
            root: Rc::new(RefCell::new(root)),
            source: source.clone(),
            wrapped_model: source.into(),
            window_handle,
        }
    }

    /// Gets the `FileModel` from the inner wrapped model.
    pub fn row_data_as_file_model(&self, index: usize) -> Option<FileModel> {
        self.wrapped_model.row_data(index)
    }

    /// Sets a new `FileModel` on the given index of the wrapped model.
    pub fn set_row_data_as_file_model(&self, index: usize, new_file_model: FileModel) {
        self.wrapped_model.set_row_data(index, new_file_model);
    }

    /// Gets the `FileModel` from the source model.
    pub fn row_data_from_source(&self, index: usize) -> Option<FileModel> {
        self.source.row_data(index)
    }

    /// Sets a new `FileModel` on the given index of the source model.
    pub fn set_row_data_source(&self, index: usize, new_file_model: FileModel) {
        self.source.set_row_data(index, new_file_model);
    }

    /// Removes the given item from the source model.
    pub fn remove_from_source(&self, file_model: FileModel) -> Option<FileModel> {
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

    pub fn set_root(&self, root: FileModel) {
        *self.root.borrow_mut() = root;
    }

    pub fn root(&self) -> FileModel {
        self.root.borrow().clone()
    }

    /// Pushes the given `FileModel` on the source model.
    pub fn push_to_source(&self, file_model: FileModel) {
        self.source.push(file_model);
    }

    /// Returns the  `row_count` of the source model.
    pub fn row_count_source(&self) -> usize {
        self.source.row_count()
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

    pub fn clear(&self) {
        self.source.set_vec(vec![]);
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
            FileType::Dir => main_window.global::<ui::Icons>().get_filled_folder(),
            FileType::Text => main_window.global::<ui::Icons>().get_filled_description(),
            FileType::Image => main_window.global::<ui::Icons>().get_filled_image(),
            FileType::Audio => main_window.global::<ui::Icons>().get_filled_audio_file(),
            FileType::Video => main_window.global::<ui::Icons>().get_filled_video_file(),
            FileType::Unknown => main_window
                .global::<ui::Icons>()
                .get_filled_insert_drive_file(),
        };
    }

    Image::default()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_files_proxy_model() {
        let proxy_model = FilesProxyModel::new(
            FileModel::new("/root"),
            vec![
                FileModel::new("/root/1.png"),
                FileModel::new("/root/2"),
                FileModel::new("/root/3"),
                FileModel::new("/root/4"),
            ],
            ui::MainWindow::new().unwrap().as_weak(),
        );

        assert_eq!(proxy_model.row_count_source(), 4);
        assert_eq!(proxy_model.row_count(), 4);

        assert_eq!(
            proxy_model.row_data_as_file_model(0),
            Some(FileModel::new("/root/1.png"))
        );
        assert_eq!(proxy_model.row_data_as_file_model(5), None);

        proxy_model.set_row_data_as_file_model(0, FileModel::new("/root/new/1.png"));
        assert_eq!(
            proxy_model.row_data_as_file_model(0),
            Some(FileModel::new("/root/new/1.png"))
        );
        assert_eq!(
            proxy_model.row_data(0),
            Some(ui::ListViewItem {
                highlighted: false,
                leading_icon: item_type_to_icon(&proxy_model.window_handle, FileType::Image),
                text: "1.png".into(),
                ..Default::default()
            })
        );

        assert_eq!(
            proxy_model.row_data_from_source(1),
            Some(FileModel::new("/root/2"))
        );

        proxy_model.set_row_data_source(1, FileModel::new("/root/new/2"));
        assert_eq!(
            proxy_model.row_data_from_source(1),
            Some(FileModel::new("/root/new/2"))
        );

        assert_eq!(
            proxy_model.remove_from_source(FileModel::new("/root/new/2")),
            Some(FileModel::new("/root/new/2"))
        );
        assert_eq!(
            proxy_model.remove_from_source(FileModel::new("/not/in/files")),
            None
        );
        assert_eq!(proxy_model.row_count_source(), 3);
        assert_eq!(proxy_model.row_count(), 3);
        assert_eq!(
            proxy_model.row_data_from_source(1),
            Some(FileModel::new("/root/3"))
        );

        assert_eq!(proxy_model.root(), FileModel::new("/root"));
        proxy_model.set_root(FileModel::new("/new_root"));
        assert_eq!(proxy_model.root(), FileModel::new("/new_root"));

        proxy_model.push_to_source(FileModel::new("/root/5"));
        assert_eq!(proxy_model.row_count_source(), 4);
        assert_eq!(proxy_model.row_count(), 4);
        assert_eq!(
            proxy_model.row_data_from_source(3),
            Some(FileModel::new("/root/5"))
        );
    }

    #[test]
    fn test_files_proxy_model_sorted() {
        let proxy_model = FilesProxyModel::new(
            FileModel::new("/root"),
            vec![
                FileModel::new("/root/5"),
                FileModel::new("/root/3"),
                FileModel::new("/root/4"),
                FileModel::new("/root/1"),
            ],
            ui::MainWindow::new().unwrap().as_weak(),
        )
        .as_sort_by(|l, r| l.name().unwrap().cmp(r.name().unwrap()));

        assert_eq!(proxy_model.row_count_source(), 4);
        assert_eq!(proxy_model.row_count(), 4);

        assert_eq!(
            proxy_model.row_data_as_file_model(0),
            Some(FileModel::new("/root/1"))
        );
        assert_eq!(
            proxy_model.row_data_from_source(0),
            Some(FileModel::new("/root/5"))
        );
        assert_eq!(
            proxy_model.row_data(0),
            Some(ui::ListViewItem {
                highlighted: false,
                leading_icon: item_type_to_icon(&proxy_model.window_handle, FileType::Dir),
                text: "1".into(),
                ..Default::default()
            })
        );

        proxy_model.set_row_data_as_file_model(3, FileModel::new("/root/0"));
        assert_eq!(
            proxy_model.row_data_as_file_model(0),
            Some(FileModel::new("/root/0"))
        );
        assert_eq!(
            proxy_model.row_data_from_source(0),
            Some(FileModel::new("/root/0"))
        );

        proxy_model.set_row_data_source(2, FileModel::new("/root/6"));
        assert_eq!(
            proxy_model.row_data_as_file_model(2),
            Some(FileModel::new("/root/3"))
        );
        assert_eq!(
            proxy_model.row_data_from_source(2),
            Some(FileModel::new("/root/6"))
        );

        assert_eq!(
            proxy_model.remove_from_source(FileModel::new("/root/6")),
            Some(FileModel::new("/root/6"))
        );

        assert_eq!(proxy_model.row_count_source(), 3);
        assert_eq!(proxy_model.row_count(), 3);

        proxy_model.push_to_source(FileModel::new("/root/9"));

        assert_eq!(proxy_model.row_count_source(), 4);
        assert_eq!(proxy_model.row_count(), 4);

        assert_eq!(
            proxy_model.row_data_as_file_model(3),
            Some(FileModel::new("/root/9"))
        );
        assert_eq!(
            proxy_model.row_data_from_source(3),
            Some(FileModel::new("/root/9"))
        );
    }

    #[test]
    fn test_files_proxy_model_filtered() {
        let proxy_model = FilesProxyModel::new(
            FileModel::new("/root"),
            vec![
                FileModel::new("/root/filtered5"),
                FileModel::new("/root/3"),
                FileModel::new("/root/filtered4"),
                FileModel::new("/root/1"),
            ],
            ui::MainWindow::new().unwrap().as_weak(),
        )
        .as_filter_by(|f| !f.name().unwrap().contains("filtered"));

        assert_eq!(proxy_model.row_count_source(), 4);
        assert_eq!(proxy_model.row_count(), 2);

        assert_eq!(
            proxy_model.row_data_as_file_model(0),
            Some(FileModel::new("/root/3"))
        );
        assert_eq!(
            proxy_model.row_data_from_source(0),
            Some(FileModel::new("/root/filtered5"))
        );
        assert_eq!(
            proxy_model.row_data(0),
            Some(ui::ListViewItem {
                highlighted: false,
                leading_icon: item_type_to_icon(&proxy_model.window_handle, FileType::Dir),
                text: "3".into(),
                ..Default::default()
            })
        );

        proxy_model.set_row_data_as_file_model(0, FileModel::new("/root/filtered3"));
        assert_eq!(proxy_model.row_count_source(), 4);
        assert_eq!(proxy_model.row_count(), 1);

        assert_eq!(
            proxy_model.row_data_as_file_model(0),
            Some(FileModel::new("/root/1"))
        );
        assert_eq!(
            proxy_model.row_data_from_source(0),
            Some(FileModel::new("/root/filtered5"))
        );

        proxy_model.push_to_source(FileModel::new("/root/9"));

        assert_eq!(proxy_model.row_count_source(), 5);
        assert_eq!(proxy_model.row_count(), 2);

        assert_eq!(
            proxy_model.row_data_as_file_model(1),
            Some(FileModel::new("/root/9"))
        );
        assert_eq!(
            proxy_model.row_data_from_source(4),
            Some(FileModel::new("/root/9"))
        );

        proxy_model.push_to_source(FileModel::new("/root/filtered9"));

        assert_eq!(proxy_model.row_count_source(), 6);
        assert_eq!(proxy_model.row_count(), 2);

        assert_eq!(
            proxy_model.row_data_as_file_model(1),
            Some(FileModel::new("/root/9"))
        );
        assert_eq!(
            proxy_model.row_data_from_source(5),
            Some(FileModel::new("/root/filtered9"))
        );

        proxy_model.remove_from_source(FileModel::new("/root/filtered9"));

        assert_eq!(proxy_model.row_count_source(), 5);
        assert_eq!(proxy_model.row_count(), 2);

        assert_eq!(
            proxy_model.row_data_as_file_model(1),
            Some(FileModel::new("/root/9"))
        );
        assert_eq!(
            proxy_model.row_data_from_source(4),
            Some(FileModel::new("/root/9"))
        );

        proxy_model.remove_from_source(FileModel::new("/root/1"));

        assert_eq!(proxy_model.row_count_source(), 4);
        assert_eq!(proxy_model.row_count(), 1);
    }
}
