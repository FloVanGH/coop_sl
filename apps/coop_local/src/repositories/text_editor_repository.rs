// SPDX-FileCopyrightText: 2023 Florian Blasius <co_sl@tutanota.com>
// SPDX-License-Identifier: GPL-3.0-only

use super::traits;
use crate::models::{FileModel, FileType, TextModel};
use std::fs;
use std::io::{self, Read, Write};

#[derive(Clone)]
pub struct TextEditorRepository;

impl Default for TextEditorRepository {
    fn default() -> Self {
        Self::new()
    }
}

impl TextEditorRepository {
    pub fn new() -> Self {
        Self
    }
}

impl traits::TextEditorRepository for TextEditorRepository {
    fn text_list(
        &self,
        parent_file_model: &FileModel,
        file_model: &FileModel,
    ) -> io::Result<Vec<FileModel>> {
        if parent_file_model.is_dir() && file_model.file_type() == FileType::Text {
            let mut images = vec![];

            for entry in fs::read_dir(parent_file_model.path())? {
                if let Some(path) = entry?.path().to_str() {
                    let child_file_model = FileModel::new(path);

                    if child_file_model.file_type() != FileType::Text {
                        continue;
                    }

                    if child_file_model.eq(file_model) {
                        images.insert(0, child_file_model);
                    } else {
                        images.push(child_file_model);
                    }
                }
            }

            return Ok(images);
        }

        Err(io::Error::new(
            io::ErrorKind::Other,
            format!("Could not load images from {}", parent_file_model.path()),
        ))
    }

    fn load_text(&self, file_model: &FileModel) -> io::Result<TextModel> {
        let mut file = file_model.as_readable_file()?;
        let mut content = String::default();

        file.read_to_string(&mut content)?;

        Ok(TextModel::new(
            file_model.name().unwrap_or_default(),
            content,
            file_model.readonly().unwrap_or_default(),
        ))
    }

    fn save(&self, file_model: &FileModel, text: &TextModel) -> io::Result<bool> {
        if file_model.readonly()? || file_model.file_type() != FileType::Text {
            return Ok(false);
        }

        let mut file = file_model.as_writeable_file()?;
        file.write_all(text.text_update().as_bytes())?;

        Ok(true)
    }

    fn remove(&self, file_model: &FileModel) -> bool {
        if let Ok(()) = trash::delete(file_model.path()) {
            return true;
        }

        false
    }
}
