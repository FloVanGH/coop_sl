// SPDX-FileCopyrightText: 2023 Florian Blasius <co_sl@tutanota.com>
// SPDX-License-Identifier: GPL-3.0-only

use crate::models::FileModel;
use std::cell::RefCell;
use std::fs;
use std::io;
use std::io::Write;
use std::path::Path;
use std::rc::Rc;

#[derive(Clone)]
pub struct FilesRepository {
    clipboard: Rc<RefCell<Vec<FileModel>>>,
}

impl Default for FilesRepository {
    fn default() -> Self {
        Self::new()
    }
}

impl FilesRepository {
    pub fn new() -> Self {
        Self {
            clipboard: Rc::new(RefCell::new(vec![])),
        }
    }

    pub fn files(&self, root: &FileModel) -> io::Result<Vec<FileModel>> {
        if root.exists() && root.is_dir() {
            let mut items = vec![];

            for entry in fs::read_dir(root.path())?.flatten() {
                items.push(FileModel::new(
                    entry.path().to_str().unwrap_or_default().to_string(),
                ));
            }

            return Ok(items);
        }

        Err(io::Error::new(
            io::ErrorKind::Other,
            format!("Cannot read files of {}", root.path()),
        ))
    }

    pub fn remove(&self, file: &FileModel) -> bool {
        if let Ok(()) = trash::delete(file.path()) {
            return true;
        }

        false
    }

    pub fn remove_all(&self, files: &[FileModel]) -> bool {
        if let Ok(()) = trash::delete_all(files.iter().map(|f| f.path())) {
            return true;
        }

        false
    }

    pub fn rename(&self, file_model: FileModel, new_name: String) -> io::Result<FileModel> {
        let new_path = Path::new(file_model.parent().unwrap_or_default()).join(new_name);

        if !new_path.exists() && fs::rename(file_model.as_path(), &new_path).is_ok() {
            return Ok(FileModel::new(new_path.to_str().unwrap_or_default()));
        }

        Err(io::Error::new(
            io::ErrorKind::Other,
            format!(
                "Cannot rename file from {} to {:?}",
                file_model.path(),
                new_path
            ),
        ))
    }

    pub fn open(&self, file_model: FileModel) -> std::io::Result<()> {
        open::that(file_model.path())
    }

    pub fn copy(&self, file: &[FileModel]) {
        *self.clipboard.borrow_mut() = file.into();
    }

    pub fn clear_clipboard(&self) {
        self.clipboard.borrow_mut().clear();
    }

    pub fn can_paste(&self) -> bool {
        // FIXME: check current root dir is not readonly
        !self.clipboard.borrow().is_empty()
    }

    pub fn paste(&self, root: &FileModel) -> io::Result<Vec<FileModel>> {
        if root.is_dir() {
            let mut files = vec![];

            for file_to_copy in self.clipboard.borrow().iter() {
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

                if file_to_copy.is_dir() {
                    copy_dir::copy_dir(file_to_copy.path(), copy_file_path.as_path())?;
                } else {
                    fs::copy(file_to_copy.path(), copy_file_path.as_path())?;
                }

                files.push(FileModel::new(copy_file_path.to_str().unwrap_or_default()));
            }

            return Ok(files);
        }

        Err(io::Error::new(
            io::ErrorKind::Other,
            format!("Cannot paste file to {}", root.path()),
        ))
    }

    pub fn create_new_folder(&self, root: &FileModel, name: &str) -> io::Result<FileModel> {
        if root.is_dir() {
            let new_folder_path = root.as_path().join(name);

            if !new_folder_path.exists() && fs::create_dir(new_folder_path.as_path()).is_ok() {
                return Ok(FileModel::new(
                    new_folder_path.as_path().to_str().unwrap_or_default(),
                ));
            }
        }

        Err(io::Error::new(
            io::ErrorKind::Other,
            format!(
                "Cannot create new folder with name {} on {}",
                name,
                root.path()
            ),
        ))
    }

    pub fn create_empty_text_file(&self, root: &FileModel, name: &str) -> io::Result<FileModel> {
        if root.is_dir() {
            let new_file_path = root.as_path().join(name);

            if !new_file_path.exists() {
                let mut new_file = fs::File::create(new_file_path.as_path())?;
                new_file.write_all(String::default().as_bytes())?;

                if let Some(path) = new_file_path.to_str() {
                    return Ok(FileModel::new(path));
                }
            }
        }

        Err(io::Error::new(
            io::ErrorKind::Other,
            format!(
                "Cannot create new file with name {} on {}",
                name,
                root.path()
            ),
        ))
    }

    pub fn contains(&self, root: &FileModel, name: &str) -> bool {
        if root.is_dir() {
            return root.as_path().join(name).exists();
        }

        false
    }
}
