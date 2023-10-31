// SPDX-FileCopyrightText: 2023 Florian Blasius <co_sl@tutanota.com>
// SPDX-License-Identifier: GPL-3.0-only

use coop_local::adapters::*;
use coop_local::controllers::*;
use coop_local::models::*;
use coop_local::repositories::*;

#[cfg(feature = "games")]
use coop_local::gamepad;

use coop_local::services::SettingsService;
use coop_local::ui::*;
use slint::*;
use std::cell::RefCell;
use std::rc::Rc;

pub fn main() -> Result<(), slint::PlatformError> {
    let main_window = MainWindow::new()?;
    let view_handle = main_window.as_weak();

    #[cfg(feature = "games")]
    let gamepad_timer = gamepad::connect(view_handle.clone())?;

    let dialog_controller = DialogController::new(view_handle.clone());
    let on_show_about = move || {
        dialog_controller.show_about();
    };

    // bookmarks
    let settings_service =
        SettingsService::new().map_err(|e| PlatformError::Other(e.to_string()))?;
    let bookmarks_repository = BookmarksRepository::new(settings_service);
    let bookmarks_controller = Rc::new(BookmarksController::new(bookmarks_repository));

    bookmarks_adapter::connect(&view_handle, &bookmarks_controller, {
        let view_handle = view_handle.clone();
        move |adapter, controller| {
            adapter.set_bookmarks(bookmarks_adapter::map_bookmarks(
                view_handle,
                controller.bookmarks(),
            ));
        }
    });
    bookmarks_adapter::connect(
        &view_handle,
        &bookmarks_controller,
        |adapter, controller| {
            adapter.on_open_dir(move |item| {
                controller.open_dir(item as usize);
            });
        },
    );
    bookmarks_adapter::connect(
        &view_handle,
        &bookmarks_controller,
        |adapter, controller| {
            adapter.on_open_next_dir(move || {
                controller.open_next_dir();
            });
        },
    );
    bookmarks_adapter::connect(
        &view_handle,
        &bookmarks_controller,
        |adapter, controller| {
            adapter.on_open_previous_dir(move || {
                controller.open_previous_dir();
            });
        },
    );
    bookmarks_adapter::connect(
        &view_handle,
        &bookmarks_controller,
        |adapter, controller| {
            adapter.on_get_item_context_menu(move |index| {
                controller.get_item_context_menu(index as usize)
            });
        },
    );
    bookmarks_adapter::connect(
        &view_handle,
        &bookmarks_controller,
        |adapter, controller| {
            adapter.on_item_context_menu_action(move |index, spec| {
                controller.execute_item_context_menu_action(index as usize, spec.as_str());
            });
        },
    );
    bookmarks_adapter::connect(
        &view_handle,
        &bookmarks_controller,
        |adapter, controller| {
            adapter.on_selected_item(move || controller.selected_item());
        },
    );
    bookmarks_adapter::connect(
        &view_handle,
        &bookmarks_controller,
        |adapter, controller| {
            adapter.on_reorder(move |source, target| {
                controller.reorder(source as usize, target as usize);
            });
        },
    );

    let initial_location = if let Some(bookmark) = bookmarks_controller.first_bookmark_path() {
        bookmark
    } else if cfg!(windows) {
        "C://".to_string()
    } else {
        "/".to_string()
    };
    bookmarks_controller.select(initial_location.as_str());
    let initial_location = FileModel::new(initial_location);

    let file_model_stack = Rc::new(RefCell::<Vec<FileModel>>::new(vec![]));

    let files_repository = FilesRepository::new();

    #[cfg(feature = "games")]
    let games_repository = GamesRepository::new();

    let files_controller = FilesController::new(
        initial_location.clone(),
        view_handle.clone(),
        files_repository.clone(),
        #[cfg(feature = "games")]
        games_repository.clone(),
    );

    let files_controller_clone = files_controller.clone();
    let back = {
        let file_model_stack = file_model_stack.clone();
        let bookmarks_controller = bookmarks_controller.clone();
        let view_handle = view_handle.clone();

        move || {
            file_model_stack.borrow_mut().pop();

            if let Some(root_file) = file_model_stack.borrow().last() {
                files_controller_clone.show_files(root_file.clone());
                bookmarks_controller.select(root_file.path());
            }

            // back is always to files view
            upgrade_adapter(&view_handle, |adapter| {
                adapter.set_active_view(View::Files);
            });

            files_controller_clone.set_back_enabled(file_model_stack.borrow().len() > 1);
        }
    };

    let on_loading = {
        let view_handle = view_handle.clone();

        move |loading| upgrade_adapter(&view_handle, move |adapter| adapter.set_loading(loading))
    };

    files_adapter::get(&view_handle, {
        let on_back = back.clone();
        move |adapter| {
            adapter.on_back(on_back);
        }
    });

    files_controller.on_show_about(on_show_about.clone());
    files_controller.on_add_bookmark({
        let bookmarks_controller = bookmarks_controller.clone();

        #[cfg(feature = "games")]
        let games_repository = games_repository.clone();

        move |file_model| {
            #[cfg(feature = "games")]
            if let Ok(is_games) = games_repository.is_games_dir(file_model.path()) {
                if is_games {
                    bookmarks_controller.add_bookmark(BookmarkModel::new(
                        coop_local::models::BookmarkType::Game,
                        file_model.name().unwrap_or_default(),
                        file_model.path(),
                        false,
                    ));
                    return;
                }
            }
            bookmarks_controller.add_bookmark(BookmarkModel::new(
                coop_local::models::BookmarkType::Dir,
                file_model.name().unwrap_or_default(),
                file_model.path(),
                false,
            ));
        }
    });

    #[cfg(feature = "games")]
    let games_controller = {
        let on_back = back.clone();
        games_adapter::get(&view_handle, |adapter| {
            adapter.on_back(on_back);
        });

        let games_controller = GamesController::new(view_handle.clone(), games_repository.clone());

        games_controller.on_show_about(on_show_about.clone());
        let files_controller_clone = files_controller.clone();

        games_controller.on_show_files({
            let file_model_stack = file_model_stack.clone();
            let view_handle = view_handle.clone();
            let bookmarks_controller = bookmarks_controller.clone();

            move || {
                if let Some(root_file) = file_model_stack.borrow().last() {
                    files_controller_clone.show_files(root_file.clone());
                    bookmarks_controller.select(root_file.path());
                }

                // back is always to files view
                upgrade_adapter(&view_handle, |adapter| {
                    adapter.set_active_view(View::Files);
                });

                files_controller_clone.set_back_enabled(file_model_stack.borrow().len() > 1);
            }
        });

        games_controller.on_loading(on_loading);

        games_controller
    };

    // text controller
    let text_input_controller = Rc::new(TextEditorController::new(TextEditorRepository::new()));

    text_editor_adapter::connect(
        &view_handle,
        &text_input_controller,
        |adapter, controller| {
            adapter.on_update_text(move |text| controller.update_text(text.as_str()));
        },
    );
    text_editor_adapter::connect(
        &view_handle,
        &text_input_controller,
        |adapter, controller| {
            adapter.on_get_context_menu(move || controller.get_context_menu());
        },
    );
    text_editor_adapter::connect(
        &view_handle,
        &text_input_controller,
        |adapter, controller| {
            adapter.on_context_menu_action({
                move |spec| controller.execute_context_menu_action(spec.as_str(), true)
            });
        },
    );
    text_editor_adapter::connect(
        &view_handle,
        &text_input_controller,
        |adapter, controller| {
            adapter.on_next(move || controller.next(true));
        },
    );
    text_editor_adapter::connect(
        &view_handle,
        &text_input_controller,
        |adapter, controller| {
            adapter.on_previous(move || controller.previous(true));
        },
    );
    text_editor_adapter::connect(
        &view_handle,
        &text_input_controller,
        |adapter, controller| {
            adapter.on_back(move || controller.invoke_back());
        },
    );
    text_input_controller.on_loading_changed({
        let view_handle = view_handle.clone();
        move |loading| {
            text_editor_adapter::set_loading(&view_handle, *loading);
        }
    });
    text_input_controller.on_single_text_changed({
        let view_handle = view_handle.clone();
        move |is_single_text| {
            text_editor_adapter::set_is_single_text(&view_handle, *is_single_text);
        }
    });
    text_input_controller.on_current_model_changed({
        let view_handle = view_handle.clone();
        move |model| {
            text_editor_adapter::set_model(&view_handle, model);
        }
    });
    text_input_controller.on_back(back.clone());
    text_input_controller.on_show_about(on_show_about.clone());

    let image_controller = ImageController::new(
        view_handle.clone(),
        files_repository,
        ImageRepository::new(),
    );
    image_controller.on_back(back.clone());
    image_controller.on_show_about(on_show_about);

    image_adapter::get(&view_handle, {
        let back = back.clone();
        move |adapter| {
            adapter.on_back(back);
        }
    });

    let files_controller_clone = files_controller.clone();
    let bookmark_controller_clone = bookmarks_controller.clone();

    let open_internal = {
        let file_model_stack = file_model_stack.clone();
        let view_handle = view_handle.clone();
        let image_controller = image_controller.clone();

        #[cfg(feature = "games")]
        let games_controller = games_controller.clone();

        move |file_model: FileModel| {
            if file_model_stack.borrow().last().eq(&Some(&file_model)) {
                return;
            }

            if file_model.is_dir() {
                file_model_stack.borrow_mut().push(file_model.clone());
                bookmark_controller_clone.select(file_model.path());

                #[cfg(feature = "games")]
                if let Ok(is_games) = games_repository.is_games_dir(file_model.path()) {
                    if is_games {
                        games_controller.show_games(file_model);
                        upgrade_adapter(&view_handle, |adapter| {
                            adapter.set_active_view(View::Games);
                        });
                        return;
                    }
                }
                files_controller_clone.show_files(file_model);
                files_controller_clone.set_back_enabled(file_model_stack.borrow().len() > 1);

                upgrade_adapter(&view_handle, |adapter| {
                    adapter.set_active_view(View::Files);
                });
            } else if file_model.file_type() == FileType::Image {
                file_model_stack.borrow_mut().push(file_model.clone());
                image_controller.load_image(file_model);
                upgrade_adapter(&view_handle, |adapter| {
                    adapter.set_active_view(View::Image);
                });
            } else if file_model.file_type() == FileType::Text {
                file_model_stack.borrow_mut().push(file_model.clone());
                text_input_controller.load(file_model, false, true);
                upgrade_adapter(&view_handle, |adapter| {
                    adapter.set_active_view(View::Text);
                });
            } else {
                files_controller_clone.open(file_model);
            }
        }
    };

    files_controller.on_open_internal(open_internal.clone());
    bookmarks_controller.on_open_internal(open_internal.clone());

    let update_timer = Timer::default();
    update_timer.start(
        TimerMode::Repeated,
        std::time::Duration::from_millis(200),
        {
            move || {
                upgrade_adapter(&view_handle, {
                    let files_controller = files_controller.clone();

                    #[cfg(feature = "games")]
                    let games_controller = games_controller.clone();

                    move |adapter| {
                        if adapter.get_active_view() == View::Files {
                            files_controller.update();
                        }

                        #[cfg(feature = "games")]
                        if adapter.get_active_view() == View::Games {
                            games_controller.update();
                        }
                    }
                });

                bookmarks_controller.update();
            }
        },
    );

    open_internal(initial_location);

    main_window.run()?;

    #[cfg(feature = "games")]
    gamepad_timer.stop();

    Ok(())
}

fn upgrade_adapter(view_handle: &Weak<MainWindow>, func: impl FnOnce(MainAdapter) + 'static) {
    if let Some(view) = view_handle.upgrade() {
        func(view.global::<MainAdapter>());
    }
}
