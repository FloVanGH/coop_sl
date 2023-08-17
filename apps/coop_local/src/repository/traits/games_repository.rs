// SPDX-FileCopyrightText: 2023 Florian Blasius <co_sl@tutanota.com>
// SPDX-License-Identifier: GPL-3.0-only
use std::io;

use crate::model::GameModel;

pub const GAMES_FILE_NAME: &str = "coop_game.toml";

pub trait GamesRepository {
    fn is_games_dir(&self, path: impl Into<String>) -> io::Result<bool>;
    fn games(&self, path: impl Into<String>) -> io::Result<Vec<GameModel>>;
    fn launch(&self, game_model: &mut GameModel) -> io::Result<()>;
}
