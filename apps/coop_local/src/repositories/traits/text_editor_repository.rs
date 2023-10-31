// SPDX-FileCopyrightText: 2023 Florian Blasius <co_sl@tutanota.com>
// SPDX-License-Identifier: GPL-3.0-only

use crate::models::{FileModel, TextModel};

use std::io;

pub trait TextEditorRepository {
    fn text_list(
        &self,
        parent_file_model: &FileModel,
        file_model: &FileModel,
    ) -> io::Result<Vec<FileModel>>;
    fn load_text(&self, file_model: &FileModel) -> io::Result<TextModel>;
    fn save(&self, file_model: &FileModel, text: &TextModel) -> io::Result<bool>;
    fn remove(&self, file_model: &FileModel) -> bool;
}
