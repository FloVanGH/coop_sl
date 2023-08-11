// SPDX-FileCopyrightText: 2023 Florian Blasius <co_sl@tutanota.com>
// SPDX-License-Identifier: GPL-3.0-only

use std::{fs, io::Result, path::Path};

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd, Eq, Ord)]
pub enum FileType {
    Dir,
    Text,
    Image,
    Audio,
    Video,
    Unknown,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct FileModel {
    path: String,
}

impl FileModel {
    pub fn new(path: impl Into<String>) -> Self {
        Self { path: path.into() }
    }

    pub fn parent(&self) -> Option<&str> {
        self.as_path().parent()?.to_str()
    }

    pub fn steam(&self) -> Option<&str> {
        self.as_path().file_stem()?.to_str()
    }

    pub fn name(&self) -> Option<&str> {
        self.as_path().file_name()?.to_str()
    }

    pub fn extension(&self) -> Option<&str> {
        self.as_path().extension()?.to_str()
    }

    pub fn file_type(&self) -> FileType {
        // FIXEM: work with test
        #[cfg(feature = "mock")]
        if self.extension().is_none() {
            return FileType::Dir;
        }

        #[cfg(not(feature = "mock"))]
        if self.as_path().is_dir() {
            return FileType::Dir;
        }

        if let Some(extension) = self.extension() {
            match extension.to_ascii_lowercase().as_str() {
                "png" | "jpg" | "jpeg" | "gif" | "ico" => return FileType::Image,
                "txt" | "slint" | "cpp" | "h" | "hpp" | "md" | "lua" | "java" | "dart" | "toml"
                | "js" | "rs" | "ts" | "json" | "yml" | "xml" | "xaml" | "cs" | "c" | "sh" => {
                    return FileType::Text
                }
                "mp3" | "acc" | "wav" | "flac" => return FileType::Audio,
                "mp4" | "mov" | "wmv" | "avi" | "avchd" | "flv" | "mpg" | "mpeg" => {
                    return FileType::Video
                }
                _ => return FileType::Unknown,
            }
        }

        FileType::Unknown
    }

    pub fn is_dir(&self) -> bool {
        self.file_type() == FileType::Dir
    }

    pub fn readonly(&self) -> Result<bool> {
        Ok(fs::metadata(self.path.as_str())?.permissions().readonly())
    }

    pub fn path(&self) -> &str {
        &self.path
    }

    pub fn exists(&self) -> bool {
        self.as_path().exists()
    }

    pub fn hidden(&self) -> bool {
        if cfg!(target_family = "unix") {
            if let Some(name) = self.name() {
                return name.starts_with(".");
            }
        }

        false
    }

    pub fn as_file(&self) -> Result<fs::File> {
        fs::File::create(self.path.as_str())
    }

    pub fn as_path(&self) -> &Path {
        Path::new(self.path.as_str())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_folder_model() {
        let folder_model = FileModel::new("/i/am/a/folder");
        assert_eq!(folder_model.parent(), Some("/i/am/a"));
        assert_eq!(folder_model.steam(), Some("folder"));
        assert_eq!(folder_model.name(), Some("folder"));
        assert_eq!(folder_model.extension(), None);
        assert_eq!(folder_model.path(), "/i/am/a/folder");
        assert_eq!(folder_model.as_path(), Path::new("/i/am/a/folder"));

        // this works only with mock features
        // assert_eq!(folder_model.file_type(), FileType::Folder);
    }

    #[test]
    fn test_image_model() {
        let folder_model = FileModel::new("/i/am/a/asset/image.png");
        assert_eq!(folder_model.parent(), Some("/i/am/a/asset"));
        assert_eq!(folder_model.steam(), Some("image"));
        assert_eq!(folder_model.name(), Some("image.png"));
        assert_eq!(folder_model.extension(), Some("png"));
        assert_eq!(folder_model.file_type(), FileType::Image);
        assert_eq!(folder_model.path(), "/i/am/a/asset/image.png");
        assert_eq!(folder_model.as_path(), Path::new("/i/am/a/asset/image.png"));
    }

    #[test]
    fn test_text_model() {
        let folder_model = FileModel::new("/i/am/a/asset/text.txt");
        assert_eq!(folder_model.parent(), Some("/i/am/a/asset"));
        assert_eq!(folder_model.steam(), Some("text"));
        assert_eq!(folder_model.name(), Some("text.txt"));
        assert_eq!(folder_model.extension(), Some("txt"));
        assert_eq!(folder_model.file_type(), FileType::Text);
        assert_eq!(folder_model.path(), "/i/am/a/asset/text.txt");
        assert_eq!(folder_model.as_path(), Path::new("/i/am/a/asset/text.txt"));
    }
}
