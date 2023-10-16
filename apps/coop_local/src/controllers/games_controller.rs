// SPDX-FileCopyrightText: 2023 Florian Blasius <co_sl@tutanota.com>
// SPDX-License-Identifier: GPL-3.0-only

use crate::models::{FileModel, GameModel};
use crate::repositories::GamesRepository;
use crate::ui::*;
use chrono::{Local, LocalResult, TimeZone};
use slint::*;
use std::cell::{Cell, RefCell};
use std::rc::Rc;
use std::time::SystemTime;
use tokio::runtime::Builder;
use tokio::sync::oneshot;

mod context_menu {
    pub const SHOW_FILES: &str = "show_files";
    pub const ABOUT: &str = "about";

    // items
    pub const SETTINGS: &str = "settings";
}

type LoadingCallback = Rc<RefCell<Box<dyn FnMut(bool) + 'static>>>;

#[derive(Clone)]
pub struct GamesController {
    view_handle: Weak<MainWindow>,
    repository: GamesRepository,
    root_file: Rc<RefCell<Option<FileModel>>>,
    root_modified: Rc<Cell<Option<SystemTime>>>,
    games: Rc<VecModel<GameModel>>,
    meta: Rc<VecModel<LauncherItem>>,
    show_about_callback: Rc<RefCell<Box<dyn FnMut() + 'static>>>,
    show_files_callback: Rc<RefCell<Box<dyn FnMut() + 'static>>>,
    loading_callback: LoadingCallback,
}

impl GamesController {
    pub fn new(view_handle: Weak<MainWindow>, repository: GamesRepository) -> Self {
        let controller = Self {
            view_handle,
            repository,
            root_file: Rc::new(RefCell::new(None)),
            root_modified: Rc::new(Cell::new(None)),
            games: Rc::new(VecModel::default()),
            meta: Rc::new(VecModel::default()),
            show_about_callback: Rc::new(RefCell::new(Box::new(|| {}))),
            show_files_callback: Rc::new(RefCell::new(Box::new(|| {}))),
            loading_callback: Rc::new(RefCell::new(Box::new(|_| {}))),
        };

        upgrade_adapter(&controller.view_handle, {
            let controller = controller.clone();

            // connect show context menu
            move |adapter| {
                adapter.set_games(
                    Rc::new(
                        controller
                            .games
                            .clone()
                            .sort_by(|l: &GameModel, r: &GameModel| {
                                r.meta().last_time_played.cmp(&l.meta().last_time_played)
                            })
                            .map(|g: GameModel| {
                                let image = if g.icon().width() == 0 || g.icon().height() == 0 {
                                    Image::default()
                                } else {
                                    Image::from_rgba8(
                                        SharedPixelBuffer::<Rgba8Pixel>::clone_from_slice(
                                            g.icon().data(),
                                            g.icon().width(),
                                            g.icon().height(),
                                        ),
                                    )
                                };

                                LauncherItem {
                                    image,
                                    text: g.name().into(),
                                }
                            }),
                    )
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
                adapter.on_get_item_context_menu({
                    let controller = controller.clone();
                    move || controller.get_item_context_menu()
                });

                adapter.on_item_context_menu_action({
                    let controller = controller.clone();
                    move |row, spec| {
                        controller.execute_item_context_menu_action(row as usize, spec.as_str())
                    }
                });
                adapter.on_launch_game(move |row| controller.launch(row as usize));
            }
        });

        upgrade_settings_adapter(&controller.view_handle, {
            let controller = controller.clone();

            move |adapter| {
                adapter.on_set_arguments({
                    let controller = controller.clone();

                    move |row, arguments| {
                        if let Some(mut game_model) = controller.games.row_data(row as usize) {
                            game_model.settings_mut().arguments = arguments.into();
                            controller.update_game_model(row as usize, game_model);
                        }
                    }
                });
                adapter.on_close(move || {
                    controller.close_game_settings();
                });
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

    pub fn on_loading(&self, callback: impl FnMut(bool) + 'static) {
        *self.loading_callback.borrow_mut() = Box::new(callback);
    }

    pub fn on_show_files(&self, callback: impl FnMut() + 'static) {
        *self.show_files_callback.borrow_mut() = Box::new(callback);
    }

    pub fn show_games(&self, file_model: FileModel) {
        *self.root_file.borrow_mut() = Some(file_model.clone());
        self.root_modified.set(file_model.modified());

        upgrade_adapter(&self.view_handle, |adapter| {
            adapter.set_loading(true);
        });

        if let Ok(games) = self.repository.games(file_model.path()) {
            self.games.set_vec(games);
        }

        upgrade_adapter(&self.view_handle, move |adapter| {
            adapter.set_current_game(0);
            adapter.set_title(file_model.name().unwrap_or_default().into());
        });

        self.display_current_meta(0);

        upgrade_adapter(&self.view_handle, |adapter| {
            adapter.set_loading(false);
        });
    }

    pub fn update(&self) {
        if let Some(root_file) = &*self.root_file.borrow() {
            if self.root_modified.get().eq(&root_file.modified()) {
                return;
            }

            let root_file = root_file.clone();
            let games_model = self.games.clone();
            let repository: GamesRepository = self.repository.clone();
            self.root_modified.set(root_file.modified());

            let _ = slint::spawn_local(async move {
                if let Ok(mut repo_games) = repository.games(root_file.path()) {
                    if repo_games.is_empty() {
                        games_model.set_vec(vec![]);
                        return;
                    }

                    for row in (0..games_model.row_count()).rev() {
                        if let Some(game_model) = games_model.row_data(row) {
                            if repo_games.contains(&game_model) {
                                repo_games.remove(
                                    repo_games.iter().position(|g| g.eq(&game_model)).unwrap(),
                                );
                                continue;
                            }

                            // game is no longer in the real directory but still in the ui (remove it)
                            games_model.remove(row);
                        }
                    }

                    for game in repo_games {
                        games_model.push(game);
                    }
                }
            });
        }
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

    fn get_item_context_menu(&self) -> ModelRc<ListViewItem> {
        VecModel::from_slice(&[ListViewItem {
            text: "Settings".into(),
            spec: context_menu::SETTINGS.into(),
            ..Default::default()
        }])
    }

    fn execute_item_context_menu_action(&self, row: usize, spec: &str) {
        if spec == context_menu::SETTINGS {
            self.open_game_settings(row);
        }
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

    fn open_game_settings(&self, row: usize) {
        if let Some(game_model) = self.games.row_data(row) {
            let arguments: SharedString = game_model.settings().arguments.as_str().into();
            let title: SharedString = game_model.name().into();

            upgrade_settings_adapter(&self.view_handle, move |adapter| {
                adapter.set_arguments(arguments);
                adapter.set_title(title);
                adapter.set_row(row as i32);
            });

            upgrade_adapter(&self.view_handle, |adapter| {
                adapter.set_display_settings(true);
            });
        }
    }

    fn update_game_model(&self, row: usize, game_model: GameModel) {
        self.repository.update_game(&game_model);
        self.games.set_row_data(row, game_model);
    }

    fn close_game_settings(&self) {
        upgrade_adapter(&self.view_handle, |adapter| {
            adapter.set_display_settings(false);
        });
    }

    fn launch(&self, row: usize) {
        let repository = self.repository.clone();
        let controller = self.clone();
        let view_handle = self.view_handle.clone();
        let loading_callback = self.loading_callback.clone();
        loading_callback.borrow_mut()(true);

        if let Some(mut game_model) = self.games.row_data(row) {
            let _ = slint::spawn_local(async move {
                let rt = Builder::new_current_thread().enable_all().build().unwrap();
                let (tx, rx) = oneshot::channel();

                let _ = std::thread::spawn(move || {
                    rt.block_on(async move {
                        let _ = repository.launch(&mut game_model);
                        let _ = tx.send(game_model);
                    });
                });

                if let Ok(game_model) = rx.await {
                    controller.games.set_row_data(row, game_model);
                    controller.display_current_meta(0);

                    upgrade_adapter(&view_handle, |adapter| {
                        adapter.set_current_game(0);
                    });

                    loading_callback.borrow_mut()(false);
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

fn upgrade_settings_adapter(
    view_handle: &Weak<MainWindow>,
    func: impl FnOnce(GameSettingsAdapter) + 'static,
) {
    if let Some(view) = view_handle.upgrade() {
        func(view.global::<GameSettingsAdapter>());
    }
}

// FIXME: load games update
