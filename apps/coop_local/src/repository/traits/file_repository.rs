// SPDX-FileCopyrightText: 2023 Florian Blasius <co_sl@tutanota.com>
// SPDX-License-Identifier: GPL-3.0-only

use crate::model::FileModel;
use std::io;

pub trait FileRepository {
    fn files(&self, root: &FileModel) -> Option<Vec<FileModel>>;
    fn remove(&self, file: &FileModel) -> bool;
    fn rename(&self, file: FileModel, new_name: String) -> Option<FileModel>;
    fn open(&self, file: FileModel) -> io::Result<()>;
    fn copy(&self, file: FileModel);
    fn can_paste(&self) -> bool;
    fn paste(&self, root: &FileModel) -> Option<FileModel>;
    fn create_new_folder(&self, root: &FileModel, name: String) -> Option<FileModel>;
}
