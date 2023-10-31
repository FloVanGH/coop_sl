// SPDX-FileCopyrightText: 2023 Florian Blasius <co_sl@tutanota.com>
// SPDX-License-Identifier: GPL-3.0-only

use super::traits;
use crate::models::{FileModel, FileType, TextModel};
use std::collections::HashMap;
use std::io;
use std::sync::{Arc, Mutex};

#[derive(Clone)]
pub struct TextEditorRepositoryMock {
    text_files: Arc<Mutex<Vec<FileModel>>>,
    text_map: Arc<Mutex<HashMap<FileModel, TextModel>>>,
}

impl TextEditorRepositoryMock {
    pub fn new(text_files: Vec<FileModel>, text_map: HashMap<FileModel, TextModel>) -> Self {
        Self {
            text_files: Arc::new(Mutex::new(text_files)),
            text_map: Arc::new(Mutex::new(text_map)),
        }
    }
}

impl traits::TextEditorRepository for TextEditorRepositoryMock {
    fn text_list(
        &self,
        _parent_file_model: &FileModel,
        _file_model: &FileModel,
    ) -> io::Result<Vec<FileModel>> {
        Ok(self.text_files.lock().unwrap().clone())
    }

    fn load_text(&self, file_model: &FileModel) -> io::Result<TextModel> {
        self.text_map
            .lock()
            .unwrap()
            .get(file_model)
            .cloned()
            .ok_or(io::Error::new(io::ErrorKind::Other, "Texts not included"))
    }

    fn save(&self, file_model: &FileModel, _text: &TextModel) -> io::Result<bool> {
        if file_model.file_type() != FileType::Text {
            return Ok(false);
        }

        Ok(true)
    }

    fn remove(&self, file_model: &FileModel) -> bool {
        let index = self
            .text_files
            .lock()
            .unwrap()
            .iter()
            .position(|f| f.eq(file_model));
        if let Some(index) = index {
            self.text_files.lock().unwrap().remove(index);
            self.text_map.lock().unwrap().remove(file_model);

            return true;
        }

        false
    }
}

#[cfg(test)]
mod tests {
    use crate::repositories::traits::TextEditorRepository;

    use super::*;

    #[test]
    fn test_text_repository_mock() {
        let file_one = FileModel::new("/text.txt");
        let file_two = FileModel::new("/text2.txt");
        let text_model_one = TextModel::new("File1.txt", "Hello world", false);
        let text_model_two = TextModel::new("File2.txt", "Hello world 2", false);

        let mut text_models = HashMap::new();
        text_models.insert(file_one.clone(), text_model_one.clone());
        text_models.insert(file_two.clone(), text_model_two.clone());

        let repository =
            TextEditorRepositoryMock::new(vec![file_one.clone(), file_two.clone()], text_models);

        assert_eq!(repository.load_text(&file_one).unwrap(), text_model_one);
        assert_eq!(repository.load_text(&file_two).unwrap(), text_model_two);
        assert!(repository.save(&file_one, &text_model_one).unwrap());
        assert!(!repository
            .save(&FileModel::new("/test.png"), &text_model_one)
            .unwrap());

        assert!(repository.remove(&file_one));
        assert!(!repository.remove(&file_one));
    }
}
