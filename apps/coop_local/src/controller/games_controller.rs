// SPDX-FileCopyrightText: 2023 Florian Blasius <co_sl@tutanota.com>
// SPDX-License-Identifier: GPL-3.0-only

use crate::model::GameModel;
use crate::proxy_models::{self, GamesProxyModel};
use crate::{repository, ui};
use slint::*;
use std::io;
use std::rc::Rc;
use tokio::runtime::Builder;
use tokio::sync::{mpsc, oneshot};

#[derive(Clone)]
pub struct GamesController {
    spawn: mpsc::Sender<GamesMessage>,
}

impl GamesController {
    pub fn new<R>(main_window: &ui::MainWindow, repository: R) -> io::Result<Self>
    where
        R: repository::traits::GamesRepository + Clone + std::marker::Send + 'static,
    {
        let rt = Builder::new_current_thread().enable_all().build()?;
        let (send, mut recv) = mpsc::channel(16);

        let controller = Self { spawn: send };

        init(&controller, &repository, main_window);

        std::thread::spawn({
            let window_handle = main_window.as_weak();

            move || {
                rt.block_on(async move {
                    while let Some(message) = recv.recv().await {
                        tokio::spawn(handle_message(
                            message,
                            repository.clone(),
                            window_handle.clone(),
                        ));
                    }
                });
            }
        });

        Ok(controller)
    }

    pub fn spawn_message(&self, message: GamesMessage) {
        let _ = self.spawn.blocking_send(message);
    }
}

fn init<R>(controller: &GamesController, repository: &R, main_window: &ui::MainWindow)
where
    R: repository::traits::GamesRepository + Clone + std::marker::Send + 'static,
{
    let adapter: ui::GamesAdapter<'_> = main_window.global::<ui::GamesAdapter>();

    adapter.on_open({
        let repository = repository.clone();
        let controller = controller.clone();

        move |path| {
            if let Ok(is_games_dir) = repository.is_games_dir(path.as_str()) {
                controller.spawn_message(GamesMessage::ShowGames { path: path.into() });

                return is_games_dir;
            }

            true
        }
    });

    adapter.on_launch_game({
        let controller = controller.clone();

        move |row| {
            controller.spawn_message(GamesMessage::Launch { row: row as usize });
        }
    });

    adapter.on_is_games_dir({
        let repository = repository.clone();

        move |path| {
            if let Ok(is_games_dir) = repository.is_games_dir(path.as_str()) {
                return is_games_dir;
            }

            false
        }
    });

    adapter.on_current_game_changed({
        let controller = controller.clone();

        move |row| {
            controller.spawn_message(GamesMessage::DisplayMeta { row: row as usize });
        }
    });
}

pub enum GamesMessage {
    ShowGames { path: String },
    Launch { row: usize },
    DisplayMeta { row: usize },
}

async fn handle_message<R>(
    message: GamesMessage,
    repository: R,
    window_handle: Weak<ui::MainWindow>,
) where
    R: repository::traits::GamesRepository + Clone + std::marker::Send + 'static,
{
    match message {
        GamesMessage::ShowGames { path } => {
            tokio::spawn(show_games(path, repository, window_handle));
        }
        GamesMessage::Launch { row } => {
            tokio::spawn(launch(row, repository, window_handle));
        }
        GamesMessage::DisplayMeta { row } => {
            tokio::spawn(display_meta(row, window_handle));
        }
    }
}

async fn show_games<R>(path: String, repository: R, window_handle: Weak<ui::MainWindow>)
where
    R: repository::traits::GamesRepository + Clone + std::marker::Send + 'static,
{
    if let Ok(games) = repository.games(path) {
        upgrade_adapter(window_handle.clone(), move |adapter| {
            let proxy_model = GamesProxyModel::new(games)
                .as_sort_by(|l, r| r.meta().last_time_played.cmp(&l.meta().last_time_played));
            adapter.set_games(Rc::new(proxy_model).into());
            adapter.set_current_game(0);
        });
    }

    let window_handle_clone = window_handle.clone();
    display_meta(0, window_handle_clone).await;
}

async fn launch<R>(row: usize, repository: R, window_handle: Weak<ui::MainWindow>)
where
    R: repository::traits::GamesRepository + Clone + std::marker::Send + 'static,
{
    let window_handle_clone = window_handle.clone();

    let (tx, rx) = oneshot::channel();

    if let Some(mut game_model) = row_data_as_game_async(row, window_handle_clone).await {
        if repository.launch(&mut game_model).is_ok() {
            upgrade_adapter(window_handle.clone(), move |adapter| {
                if let Some(proxy_model) = adapter
                    .get_games()
                    .as_any()
                    .downcast_ref::<proxy_models::GamesProxyModel>()
                {
                    proxy_model.set_row_data_as_game(row, game_model.clone());

                    if let Some(row) = proxy_model.row(&game_model) {
                        adapter.set_current_game(row as i32);
                        let _ = tx.send(row);
                    }
                }
            });
        }
    }

    if let Ok(row) = rx.await {
        let window_handle_clone = window_handle.clone();
        display_meta(row, window_handle_clone).await;
    }
}

async fn display_meta(row: usize, window_handle: Weak<ui::MainWindow>) {
    use chrono::*;

    let _ = window_handle.upgrade_in_event_loop(move |main_window| {
        if let Some(proxy_model) = main_window
            .global::<ui::GamesAdapter>()
            .get_games()
            .as_any()
            .downcast_ref::<proxy_models::GamesProxyModel>()
        {
            if let Some(game_model) = proxy_model.row_data_as_game(row) {
                let mut last_time_played = "--".to_string();

                if game_model.meta().last_time_played > 0 {
                    if let LocalResult::Single(date) =
                        Local.timestamp_millis_opt(game_model.meta().last_time_played)
                    {
                        // FIXME: localization
                        last_time_played = date.format("%Y-%m-%d %H:%M").to_string();
                    };
                }

                let mut play_time = "00:00".to_string();
                if game_model.meta().play_time > 0 {
                    play_time = std::format!(
                        "{:02}:{:02}",
                        game_model.meta().play_time / 3600 % 24,
                        game_model.meta().play_time / 60 % 60
                    )
                }

                main_window
                    .global::<ui::GamesAdapter>()
                    .set_current_game_meta(VecModel::from_slice(&[
                        ui::LauncherItem {
                            text: std::format!("Last time played: {}", last_time_played).into(),
                            image: main_window
                                .global::<ui::Icons>()
                                .get_filled_calendar_month(),
                        },
                        ui::LauncherItem {
                            text: std::format!("Times played: {}", game_model.meta().times_played)
                                .into(),
                            image: main_window.global::<ui::Icons>().get_add(),
                        },
                        ui::LauncherItem {
                            text: std::format!("Play time: {}", play_time).into(),
                            image: main_window.global::<ui::Icons>().get_filled_av_timer(),
                        },
                    ]));
            }
        }
    });
}

// helper

pub async fn is_games_view_visible_async(window_handle: Weak<ui::MainWindow>) -> bool {
    let (tx, rx) = oneshot::channel();

    let _ = window_handle.upgrade_in_event_loop(move |main_window| {
        let _ = tx.send(
            main_window
                .global::<ui::FilesAdapter>()
                .get_games_view_visible(),
        );
    });

    if let Ok(is_visible) = rx.await {
        return is_visible;
    }

    false
}

pub async fn row_data_as_game_async(
    row: usize,
    window_handle: Weak<ui::MainWindow>,
) -> Option<GameModel> {
    let (tx, rx) = oneshot::channel();

    upgrade_proxy_model(window_handle, move |proxy_model| {
        let _ = tx.send(proxy_model.row_data_as_game(row));
    });

    if let Ok(game) = rx.await {
        return game;
    }

    None
}

fn upgrade_adapter(
    window_handle: Weak<ui::MainWindow>,
    func: impl FnOnce(ui::GamesAdapter) + Send + 'static,
) {
    let _ = window_handle
        .upgrade_in_event_loop(move |main_window| func(main_window.global::<ui::GamesAdapter>()));
}

fn upgrade_proxy_model(
    window_handle: Weak<ui::MainWindow>,
    func: impl FnOnce(&GamesProxyModel) + Send + 'static,
) {
    upgrade_adapter(window_handle, move |adapter| {
        if let Some(proxy_model) = adapter
            .get_games()
            .as_any()
            .downcast_ref::<proxy_models::GamesProxyModel>()
        {
            func(proxy_model);
        }
    });
}
