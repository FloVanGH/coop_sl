// SPDX-FileCopyrightText: 2023 Florian Blasius <co_sl@tutanota.com>
// SPDX-License-Identifier: GPL-3.0-only

use crate::models::{FileModel, FileType};
use image::DynamicImage;
use std::fs;
use std::io;

#[derive(Clone)]
pub struct ImageRepository;

impl Default for ImageRepository {
    fn default() -> Self {
        Self::new()
    }
}

impl ImageRepository {
    pub fn new() -> Self {
        Self
    }

    pub async fn image_list(
        &self,
        parent_file_model: &FileModel,
        file_model: &FileModel,
    ) -> io::Result<Vec<FileModel>> {
        if parent_file_model.is_dir() && file_model.file_type() == FileType::Image {
            let mut images = vec![];

            for entry in fs::read_dir(parent_file_model.path())? {
                if let Some(path) = entry?.path().to_str() {
                    let child_file_model = FileModel::new(path);

                    if child_file_model.file_type() != FileType::Image {
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

    pub async fn load_image(&self, file_model: &FileModel) -> io::Result<DynamicImage> {
        image::open(file_model.path()).map_err(|_| {
            io::Error::new(
                io::ErrorKind::Other,
                format!("Could not load image {}", file_model.path()),
            )
        })
    }
}
