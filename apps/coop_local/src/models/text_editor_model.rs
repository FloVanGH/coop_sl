// SPDX-FileCopyrightText: 2023 Florian Blasius <co_sl@tutanota.com>
// SPDX-License-Identifier: GPL-3.0-only

use std::{
    cell::{Cell, RefCell},
    collections::HashMap,
};

use super::{FileModel, TextModel};

#[derive(Default)]
pub struct TextEditorModel {
    pub current_index: Cell<usize>,
    pub files: RefCell<Vec<FileModel>>,
    pub text_cache: RefCell<HashMap<FileModel, TextModel>>,
}

impl TextEditorModel {
    pub fn current_index(&self) -> usize {
        self.current_index.get()
    }

    pub fn set_current_index(&self, index: usize) {
        self.current_index.set(index);
    }

    pub fn current_file(&self) -> Option<FileModel> {
        self.files.borrow().get(self.current_index.get()).cloned()
    }

    pub fn get_file(&self, index: usize) -> Option<FileModel> {
        self.files.borrow().get(index).cloned()
    }

    pub fn remove(&self, index: usize) {
        if index >= self.len() {
            return;
        }

        self.text_cache
            .borrow_mut()
            .remove(&self.files.borrow_mut().remove(index));
    }

    pub fn get_text(&self, file: &FileModel) -> Option<TextModel> {
        self.text_cache.borrow().get(file).cloned()
    }

    pub fn set_text(&self, file: FileModel, text: TextModel) {
        self.text_cache.borrow_mut().insert(file, text);
    }

    pub fn clear(&self) {
        self.current_index.set(0);
        self.files.borrow_mut().clear();
        self.text_cache.borrow_mut().clear();
    }

    pub fn append(&self, files: &mut Vec<FileModel>) {
        self.files.borrow_mut().append(files);
    }

    pub fn len(&self) -> usize {
        self.files.borrow().len()
    }

    pub fn is_empty(&self) -> bool {
        self.files.borrow().is_empty()
    }
}

#[cfg(test)]
mod tests {
    pub use super::*;

    fn model() -> TextEditorModel {
        let model = TextEditorModel::default();
        model.append(&mut vec![
            FileModel::new("/test/file.slint"),
            FileModel::new("/test/file1.slint"),
            FileModel::new("/test/file2.slint"),
            FileModel::new("/test/file3.slint"),
        ]);
        model
    }

    #[test]
    fn test_files() {
        let model = model();

        assert_eq!(model.current_index(), 0);
        model.set_current_index(1);
        assert_eq!(model.current_index(), 1);
        assert_eq!(
            model.current_file(),
            Some(FileModel::new("/test/file1.slint"))
        );
        model.set_current_index(10);
        assert_eq!(model.current_file(), None);
        assert_eq!(model.get_file(3), Some(FileModel::new("/test/file3.slint")));
    }

    #[test]
    fn test_text() {
        let model = model();
        model.set_text(
            FileModel::new("/test/file.slint"),
            TextModel::new("file.slint", "hello there", false),
        );
        assert_eq!(
            model.get_text(&FileModel::new("/test/file.slint")),
            Some(TextModel::new("file.slint", "hello there", false))
        );
        assert_eq!(model.get_text(&FileModel::new("/test/file1.slint")), None);

        model.remove(0);
        assert_eq!(model.get_text(&FileModel::new("/test/file.slint")), None);
        assert_eq!(model.get_file(0), Some(FileModel::new("/test/file1.slint")));
    }

    #[test]
    fn test_clear() {
        let model = model();
        assert!(!model.is_empty());
        assert_eq!(model.len(), 4);

        model.clear();
        assert!(model.is_empty());
        assert_eq!(model.len(), 0);
    }
}
