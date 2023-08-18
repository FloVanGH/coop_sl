// SPDX-FileCopyrightText: 2023 Florian Blasius <co_sl@tutanota.com>
// SPDX-License-Identifier: GPL-3.0-only

use crate::model::CoopGameModel;
use crate::model::GameMetaModel;
use crate::model::GameModel;
use crate::service;

use super::traits;
use std::fs;
use std::io;
use std::process::Command;
use std::sync::{Arc, Mutex};

#[cfg(target_os = "macos")]
pub const MACOS_ICON_PATH: &str = "Contents/Resources/";

#[cfg(target_os = "macos")]
pub const MACOS_ICON_EXT: &str = "icns";

#[cfg(target_os = "macos")]
pub const MACOS_APP_EXT: &str = "app";

#[cfg(target_os = "linux")]
pub const APP_IMAGE_EXT: &str = "appimage";

pub const COOP_GAME: &str = "coop_game";

#[derive(Clone, Debug)]
pub struct GamesRepository {
    coop_game_model: Arc<Mutex<CoopGameModel>>,
    current_path: Arc<Mutex<String>>,
}

impl Default for GamesRepository {
    fn default() -> Self {
        Self::new()
    }
}

impl GamesRepository {
    pub fn new() -> Self {
        Self {
            coop_game_model: Arc::new(Mutex::new(CoopGameModel::default())),
            current_path: Arc::new(Mutex::new(String::default())),
        }
    }

    #[cfg(target_os = "macos")]
    fn get_games(&self, path: impl Into<String>) -> io::Result<Vec<GameModel>> {
        use crate::model::{FileModel, RgbaIconModel};

        let mut games = vec![];

        let path = path.into();

        for entry in fs::read_dir(path)?.flatten() {
            let file_model = FileModel::from(entry.path().to_str().unwrap_or_default());

            if !file_model.extension().eq(&Some(MACOS_APP_EXT)) {
                continue;
            }

            let mut icon = RgbaIconModel::default();

            for resource_entry in
                fs::read_dir(file_model.as_path().join(MACOS_ICON_PATH))?.flatten()
            {
                let resource_file_model =
                    FileModel::from(resource_entry.path().to_str().unwrap_or_default());

                if !resource_file_model.extension().eq(&Some(MACOS_ICON_EXT)) {
                    continue;
                }

                if let Ok(icon_file) = resource_file_model.as_readable_file() {
                    let buffered = io::BufReader::new(icon_file);

                    if let Ok(family) = tauri_icns::IconFamily::read(buffered) {
                        let mut icon_types = family.available_icons();
                        icon_types.sort_by_key(|l| l.screen_width());
                        icon_types.reverse();

                        for icon_type in icon_types {
                            if let Ok(image) = family.get_icon_with_type(icon_type) {
                                let image = image.convert_to(tauri_icns::PixelFormat::RGBA);

                                icon = RgbaIconModel::new(image.width(), image.data());
                                break;
                            }
                        }
                    }

                    break;
                }
            }

            let name = file_model.steam().unwrap_or_default().to_string();

            games.push(GameModel::new(file_model, icon, self.get_meta(&name)?));
        }

        Ok(games)
    }

    #[cfg(target_os = "linux")]
    fn get_games(&self, path: impl Into<String>) -> io::Result<Vec<GameModel>> {
        use crate::model::{FileModel, RgbaIconModel};
        let mut games = vec![];

        let path = path.into();

        for entry in fs::read_dir(path)?.flatten() {
            let file_model = FileModel::from(entry.path().to_str().unwrap_or_default());

            if !file_model.extension().eq(&Some(APP_IMAGE_EXT)) {
                continue;
            }

            let name = file_model.steam().unwrap_or_default().to_string();

            // FIXME read icon from app image
            games.push(GameModel::new(
                file_model,
                RgbaIconModel::default(),
                self.get_meta(&name)?,
            ));
        }

        Ok(games)
    }

    #[cfg(all(not(target_os = "linux"), not(target_os = "macos")))]
    fn get_games(&self, path: impl Into<String>) -> io::Result<Vec<GameModel>> {
        Ok(vec![])
    }

    fn get_meta(&self, name: &str) -> io::Result<GameMetaModel> {
        if let Ok(mut coop_game_model) = self.coop_game_model.lock() {
            if !coop_game_model.meta.contains_key(name) {
                coop_game_model
                    .meta
                    .insert(name.to_string(), GameMetaModel::default());
            }

            return Ok(coop_game_model.meta.get(name).cloned().unwrap());
        }

        Ok(GameMetaModel::default())
    }

    fn save_coop_game_model(&self) {
        if let Ok(current_path) = self.current_path.lock() {
            if let Ok(coop_game_model) = self.coop_game_model.lock() {
                let _ = service::save(&current_path, COOP_GAME, &*coop_game_model);
            }
        }
    }
}

impl traits::GamesRepository for GamesRepository {
    fn is_games_dir(&self, path: impl Into<String>) -> io::Result<bool> {
        let path = path.into();

        for entry in fs::read_dir(path)?.flatten() {
            if let Some(name) = entry.file_name().to_str() {
                if name.eq(traits::GAMES_FILE_NAME) {
                    return Ok(true);
                }
            }
        }

        Ok(false)
    }

    fn games(&self, path: impl Into<String>) -> io::Result<Vec<GameModel>> {
        let path = path.into();

        if let Ok(mut current_path) = self.current_path.lock() {
            *current_path = path.clone();
        }

        if let Ok(mut coop_game_model) = self.coop_game_model.lock() {
            if let Ok(loaded_model) = service::load(path.as_str(), COOP_GAME) {
                *coop_game_model = loaded_model;
            }
        }

        self.get_games(path)
    }

    fn launch(&self, game_model: &mut GameModel) -> io::Result<()> {
        use chrono::*;
        use std::time;

        game_model.meta_mut().times_played += 1;
        game_model.meta_mut().last_time_played = Local::now().timestamp_millis();

        let start_time = time::Instant::now();

        #[cfg(target_os = "macos")]
        if let Ok(join_handle) = Command::new("open")
            .arg("--wait-apps")
            .arg(game_model.file_model().path())
            .spawn()
        {
            let _ = join_handle.wait_with_output();
        }

        #[cfg(not(target_os = "macos"))]
        if let Ok(join_handle) = Command::new(game_model.file_model().path()).spawn() {
            let _ = join_handle.wait_with_output();
        }

        game_model.meta_mut().play_time += start_time.elapsed().as_secs();

        if let Ok(mut coop_game_model) = self.coop_game_model.lock() {
            coop_game_model
                .meta
                .insert(game_model.name().to_string(), game_model.meta().clone());
        }

        self.save_coop_game_model();
        Ok(())
    }
}
