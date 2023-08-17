// SPDX-FileCopyrightText: 2023 Florian Blasius <co_sl@tutanota.com>
// SPDX-License-Identifier: GPL-3.0-only

use crate::model::GameModel;
use std::io;

use super::traits;

#[derive(Clone, Debug)]
pub struct GamesRepositoryMock {}

impl GamesRepositoryMock {
    pub fn new() -> Self {
        Self {}
    }
}

impl Default for GamesRepositoryMock {
    fn default() -> Self {
        Self::new()
    }
}

impl traits::GamesRepository for GamesRepositoryMock {
    fn is_games_dir(&self, path: impl Into<String>) -> io::Result<bool> {
        let path = path.into();

        Ok(path.to_uppercase().contains("games"))
    }

    fn games(&self, _path: impl Into<String>) -> io::Result<Vec<GameModel>> {
        let games = vec![];

        Ok(games)
    }

    fn launch(&self, game_model: &mut GameModel) -> io::Result<()> {
        println!("Launch game: {}", game_model.name());

        Ok(())
    }
}
