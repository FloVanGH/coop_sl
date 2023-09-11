// SPDX-FileCopyrightText: 2023 Florian Blasius <co_sl@tutanota.com>
// SPDX-License-Identifier: GPL-3.0-only

use crate::models::{FileModel, GameModel};
use crate::proxy_model::ProxyModel;
use crate::repositories::GamesRepository;
use crate::ui::*;
use chrono::{Local, LocalResult, TimeZone};
use slint::*;
use std::cell::RefCell;
use std::rc::Rc;

mod context_menu {
    pub const SHOW_FILES: &str = "show_files";
    pub const ABOUT: &str = "about";
}

#[derive(Clone)]
pub struct GamesController {
    view_handle: Weak<MainWindow>,
    repository: GamesRepository,

    games: Rc<ProxyModel<GameModel>>,
    meta: Rc<VecModel<LauncherItem>>,
    show_about_callback: Rc<RefCell<Box<dyn FnMut() + 'static>>>,
    show_files_callback: Rc<RefCell<Box<dyn FnMut() + 'static>>>,
}

impl GamesController {
    pub fn new(view_handle: Weak<MainWindow>, repository: GamesRepository) -> Self {
        // let games = ;

        let controller = Self {
            view_handle,
            repository,

            games: Rc::new(
                ProxyModel::default().as_sort_by(|l: &GameModel, r: &GameModel| {
                    r.meta().last_time_played.cmp(&l.meta().last_time_played)
                }),
            ),
            meta: Rc::new(VecModel::default()),
            show_about_callback: Rc::new(RefCell::new(Box::new(|| {}))),
            show_files_callback: Rc::new(RefCell::new(Box::new(|| {}))),
        };

        upgrade_adapter(&controller.view_handle, {
            let controller = controller.clone();

            // connect show context menu
            move |adapter| {
                adapter.set_games(
                    Rc::new(controller.games.clone().map(|g: GameModel| {
                        let image = if g.icon().width() == 0 || g.icon().height() == 0 {
                            Image::default()
                        } else {
                            Image::from_rgba8(SharedPixelBuffer::<Rgba8Pixel>::clone_from_slice(
                                g.icon().data(),
                                g.icon().width(),
                                g.icon().height(),
                            ))
                        };

                        LauncherItem {
                            image,
                            text: g.name().into(),
                        }
                    }))
                    .into(),
                );
                adapter.set_current_game_meta(controller.meta.clone().into());
                adapter.on_get_context_menu({
                    let controller = controller.clone();
                    move || controller.get_context_menu()
                });

                adapter.on_context_menu_action({
                    let controller = controller.clone();
                    move |spec| controller.execute_context_menu_action(spec.as_str())
                });
                adapter.on_current_game_changed({
                    let controller = controller.clone();
                    move |row| controller.display_current_meta(row as usize)
                });
                adapter.on_launch_game(move |row| controller.launch(row as usize))
            }
        });

        controller
    }

    pub fn on_back(&self, func: impl FnMut() + 'static) {
        upgrade_adapter(&self.view_handle, move |adapter| {
            adapter.on_back(func);
        });
    }

    pub fn on_show_about(&self, callback: impl FnMut() + 'static) {
        *self.show_about_callback.borrow_mut() = Box::new(callback);
    }

    pub fn on_show_files(&self, callback: impl FnMut() + 'static) {
        *self.show_files_callback.borrow_mut() = Box::new(callback);
    }

    pub fn show_games(&self, file_model: FileModel) {
        if let Ok(games) = self.repository.games(file_model.path()) {
            self.games.set_vec_to_source(games);
        }

        upgrade_adapter(&self.view_handle, move |adapter| {
            adapter.set_current_game(0);
            adapter.set_title(file_model.name().unwrap_or_default().into());
        });

        self.display_current_meta(0);
    }

    fn display_current_meta(&self, row: usize) {
        if let Some(game_model) = self.games.row_data(row) {
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

            self.meta.set_vec(vec![
                LauncherItem {
                    text: std::format!("Last time played: {}", last_time_played).into(),
                    image: self
                        .view_handle
                        .upgrade()
                        .map(|v| v.global::<Icons>().get_filled_calendar_month())
                        .unwrap_or_default(),
                },
                LauncherItem {
                    text: std::format!("Times played: {}", game_model.meta().times_played).into(),
                    image: self
                        .view_handle
                        .upgrade()
                        .map(|v| v.global::<Icons>().get_add())
                        .unwrap_or_default(),
                },
                LauncherItem {
                    text: std::format!("Play time: {}", play_time).into(),
                    image: self
                        .view_handle
                        .upgrade()
                        .map(|v| v.global::<Icons>().get_filled_av_timer())
                        .unwrap_or_default(),
                },
            ])
        }
    }

    fn get_context_menu(&self) -> ModelRc<ListViewItem> {
        VecModel::from_slice(&[
            ListViewItem {
                text: "Show files".into(),
                spec: context_menu::SHOW_FILES.into(),
                ..Default::default()
            },
            ListViewItem {
                text: "About".into(),
                spec: context_menu::ABOUT.into(),
                ..Default::default()
            },
        ])
    }

    fn execute_context_menu_action(&self, spec: &str) {
        match spec {
            context_menu::SHOW_FILES => {
                if let Ok(mut callback) = self.show_files_callback.try_borrow_mut() {
                    callback();
                }
            }
            context_menu::ABOUT => {
                if let Ok(mut callback) = self.show_about_callback.try_borrow_mut() {
                    callback();
                }
            }
            _ => {}
        }
    }

    fn launch(&self, row: usize) {
        let repository = self.repository.clone();
        let controller = self.clone();
        let view_handle = self.view_handle.clone();

        if let Some(mut game_model) = self.games.row_data(row) {
            let _ = slint::spawn_local(async move {
                if let Ok(game_model) = std::thread::spawn(move || {
                    let _ = repository.launch(&mut game_model);
                    game_model
                })
                .join()
                {
                    // FIXME: block games view with loading indicator until game is closed
                    controller.games.set_row_data(row, game_model);
                    controller.display_current_meta(row);

                    upgrade_adapter(&view_handle, |adapter| {
                        adapter.set_current_game(0);
                    })
                }
            });
        }
    }
}

fn upgrade_adapter(view_handle: &Weak<MainWindow>, func: impl FnOnce(GamesAdapter) + 'static) {
    if let Some(view) = view_handle.upgrade() {
        func(view.global::<GamesAdapter>());
    }
}

// FIXME: load games update
