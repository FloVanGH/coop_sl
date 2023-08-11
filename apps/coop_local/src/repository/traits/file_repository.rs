// SPDX-FileCopyrightText: 2023 Florian Blasius <co_sl@tutanota.com>
// SPDX-License-Identifier: GPL-3.0-only

use crate::model::FileModel;
use std::io;

pub trait FileRepository {
    fn files(&self, root: &FileModel) -> io::Result<Vec<FileModel>>;
    fn remove(&self, file: &FileModel) -> bool;
    fn rename(&self, file: FileModel, new_name: String) -> io::Result<FileModel>;
    fn open(&self, file: FileModel) -> io::Result<()>;
    fn add_to_clipboard(&self, file: FileModel);
    fn clear_clipboard(&self);
    fn can_paste(&self) -> bool;
    fn paste(&self, root: &FileModel) -> io::Result<Vec<FileModel>>;
    fn create_new_folder(&self, root: &FileModel, name: String) -> io::Result<FileModel>;
}
