// SPDX-FileCopyrightText: 2023 Florian Blasius <co_sl@tutanota.com>
// SPDX-License-Identifier: GPL-3.0-only

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use super::FileModel;

#[derive(Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct RgbaIconModel {
    width: u32,
    height: u32,
    data: Vec<u8>,
}

impl RgbaIconModel {
    pub fn new(width: u32, data: &[u8]) -> Self {
        Self {
            width,
            height: data.len() as u32 / 4 / width,
            data: Vec::from(data),
        }
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn data(&self) -> &[u8] {
        &self.data
    }
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct GameMetaModel {
    pub times_played: u32,
    pub last_time_played: i64,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct CoopGameModel {
    pub meta: HashMap<String, GameMetaModel>,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct GameModel {
    file_model: FileModel,
    icon_model: RgbaIconModel,
    meta_model: GameMetaModel,
}

impl GameModel {
    pub fn new(
        file_model: impl Into<FileModel>,
        icon_model: RgbaIconModel,
        meta_model: GameMetaModel,
    ) -> Self {
        Self {
            file_model: file_model.into(),
            icon_model,
            meta_model,
        }
    }

    pub fn name(&self) -> &str {
        self.file_model.steam().unwrap_or_default()
    }

    pub fn file_model(&self) -> &FileModel {
        &self.file_model
    }

    pub fn icon(&self) -> &RgbaIconModel {
        &self.icon_model
    }

    pub fn meta(&self) -> &GameMetaModel {
        &self.meta_model
    }

    pub fn meta_mut(&mut self) -> &mut GameMetaModel {
        &mut self.meta_model
    }
}

#[cfg(test)]
mod tests {
    pub use super::*;

    #[test]
    fn test_icon_model() {
        let icon_model = RgbaIconModel::new(5, &[0; 5 * 5 * 4]);

        assert_eq!(icon_model.width(), 5);
        assert_eq!(icon_model.height(), 5);
        assert_eq!(icon_model.data(), &vec![0; 5 * 5 * 4]);
    }

    #[test]
    fn test_game_model() {
        assert_eq!(
            GameModel::new(
                "/test/the/name.app",
                RgbaIconModel::default(),
                GameMetaModel::default()
            )
            .name(),
            "name"
        );
        assert_eq!(
            GameModel::new("", RgbaIconModel::default(), GameMetaModel::default()).name(),
            ""
        );
    }
}
