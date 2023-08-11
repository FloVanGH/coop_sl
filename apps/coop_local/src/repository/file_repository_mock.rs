// SPDX-FileCopyrightText: 2023 Florian Blasius <co_sl@tutanota.com>
// SPDX-License-Identifier: GPL-3.0-only

use super::traits;
use crate::model::{self, FileModel};
use std::io;
use std::sync::{Arc, Mutex};

#[derive(Clone)]
pub struct FileRepositoryMock {
    file_to_copy: Arc<Mutex<Option<FileModel>>>,
}

impl Default for FileRepositoryMock {
    fn default() -> Self {
        Self::new()
    }
}

impl FileRepositoryMock {
    pub fn new() -> Self {
        Self {
            file_to_copy: Arc::new(Mutex::new(None)),
        }
    }
}

impl traits::FileRepository for FileRepositoryMock {
    fn files(&self, root: &FileModel) -> io::Result<Vec<FileModel>> {
        Ok(match root.path() {
            "/docs" => {
                let mut items = vec![FileModel::new("/docs/basics")];

                items.append(
                    &mut (0..500)
                        .map(|i| model::FileModel::new(format!("/docs/file_{}.md", i)))
                        .collect(),
                );
                items
            }
            "/docs/basics" => (0..500)
                .map(|i| model::FileModel::new(format!("/docs/basics/file_{}.md", i)))
                .collect(),
            "/src" => (0..500)
                .map(|i| model::FileModel::new(format!("/src/file_{}.slint", i)))
                .collect(),
            _ => vec![
                model::FileModel::new("/docs"),
                model::FileModel::new("/src"),
                model::FileModel::new("/README.md"),
            ],
        })
    }

    fn remove(&self, _file: &FileModel) -> bool {
        true
    }

    fn rename(&self, _file_model: FileModel, new_name: String) -> io::Result<FileModel> {
        Ok(FileModel::new(new_name))
    }

    fn open(&self, file_model: FileModel) -> std::io::Result<()> {
        println!("Open file: {}", file_model.path());

        Ok(())
    }

    fn copy(&self, file: FileModel) {
        if let Ok(mut copy_file) = self.file_to_copy.lock() {
            *copy_file = Some(file);
        }
    }

    fn can_paste(&self) -> bool {
        if let Ok(copy_file) = self.file_to_copy.lock() {
            return (*copy_file).is_some();
        }

        false
    }

    fn paste(&self, root: &FileModel) -> io::Result<FileModel> {
        if let Ok(copy_file) = self.file_to_copy.lock() {
            if let Some(copy_file) = (*copy_file).as_ref() {
                return Ok(copy_file.clone());
            }
        }

        Err(io::Error::new(
            io::ErrorKind::Other,
            format!("Cannot paste file to {}", root.path()),
        ))
    }

    fn create_new_folder(&self, root: &FileModel, name: String) -> io::Result<FileModel> {
        Ok(FileModel::new(
            root.as_path()
                .join(name)
                .as_path()
                .to_str()
                .unwrap_or_default(),
        ))
    }
}
