// SPDX-FileCopyrightText: 2023 Florian Blasius <co_sl@tutanota.com>
// SPDX-License-Identifier: GPL-3.0-only

use super::traits;
use crate::model::FileModel;
use std::fs;
use std::sync::{Arc, Mutex};

#[derive(Clone)]
pub struct FileRepository {
    file_to_copy: Arc<Mutex<Option<FileModel>>>,
}

impl Default for FileRepository {
    fn default() -> Self {
        Self::new()
    }
}

impl FileRepository {
    pub fn new() -> Self {
        Self {
            file_to_copy: Arc::new(Mutex::new(None)),
        }
    }
}

impl traits::FileRepository for FileRepository {
    fn files(&self, root: &FileModel) -> Option<Vec<FileModel>> {
        if root.is_dir() {
            if let Ok(paths) = fs::read_dir(root.path()) {
                let mut items = vec![];

                for entry in paths.flatten() {
                    items.push(FileModel::new(
                        entry.path().to_str().unwrap_or_default().to_string(),
                    ));
                }

                return Some(items);
            }
        }

        None
    }

    fn remove(&self, file: &FileModel) -> bool {
        if let Ok(()) = trash::delete(file.path()) {
            return true;
        }

        false
    }

    fn rename(&self, file_model: FileModel, new_name: String) -> Option<FileModel> {
        if fs::rename(file_model.as_path(), new_name.as_str()).is_ok() {
            return Some(FileModel::new(new_name));
        }

        None
    }

    fn open(&self, file_model: FileModel) -> std::io::Result<()> {
        open::that(file_model.path())
    }

    fn copy(&self, file: FileModel) {
        if let Ok(mut copy_file) = self.file_to_copy.lock() {
            *copy_file = Some(file);
        }
    }

    fn can_paste(&self) -> bool {
        // FIXME: check current root dir is not readonly
        if let Ok(copy_file) = self.file_to_copy.lock() {
            return (*copy_file).is_some();
        }

        false
    }

    fn paste(&self, root: &FileModel) -> Option<FileModel> {
        if !root.is_dir() {
            return None;
        }

        if let Ok(file_to_copy) = self.file_to_copy.lock() {
            if let Some(file_to_copy) = (*file_to_copy).as_ref() {
                let mut copy_file_path =
                    root.as_path().join(file_to_copy.name().unwrap_or_default());
                let mut counter: i32 = -1;

                loop {
                    if !copy_file_path.exists() {
                        break;
                    }

                    counter += 1;

                    if counter == 0 {
                        copy_file_path = root.as_path().join(format!(
                            "{}-copy.{}",
                            file_to_copy.steam().unwrap_or_default(),
                            file_to_copy.extension().unwrap_or_default()
                        ));

                        continue;
                    }

                    copy_file_path = root.as_path().join(format!(
                        "{}-copy-{}.{}",
                        file_to_copy.steam().unwrap_or_default(),
                        counter,
                        file_to_copy.extension().unwrap_or_default()
                    ));
                }

                if fs::copy(file_to_copy.path(), copy_file_path.as_path()).is_ok() {
                    return Some(FileModel::new(copy_file_path.to_str().unwrap_or_default()));
                }
            }
        }

        None
    }

    fn create_new_folder(&self, root: &FileModel, name: String) -> Option<FileModel> {
        if !root.is_dir() {
            return None;
        }

        let new_folder_path = root.as_path().join(name);

        if fs::create_dir(new_folder_path.as_path()).is_ok() {
            return Some(FileModel::new(
                new_folder_path.as_path().to_str().unwrap_or_default(),
            ));
        }

        None
    }
}
