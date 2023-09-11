// SPDX-FileCopyrightText: 2023 Florian Blasius <co_sl@tutanota.com>
// SPDX-License-Identifier: GPL-3.0-only

use coop_local::controllers::{
    BookmarksController, DialogController, FilesController, GamesController, ImageController,
    TextController,
};
use coop_local::models::{BookmarkModel, FileModel, FileType};
use coop_local::repositories::{FilesRepository, GamesRepository};
use coop_local::ui::*;
use slint::*;
use std::cell::RefCell;
use std::rc::Rc;

pub fn main() -> Result<(), slint::PlatformError> {
    let main_window = MainWindow::new()?;
    let view_handle = main_window.as_weak();

    let dialog_controller = DialogController::new(view_handle.clone());
    let on_show_about = move || {
        dialog_controller.show_about();
    };

    let bookmarks_controller = BookmarksController::new(view_handle.clone())
        .map_err(|e| PlatformError::Other(e.to_string()))?;

    let file_model_stack = Rc::new(RefCell::new(vec![FileModel::new("/")]));

    let files_repository = FilesRepository::new();
    let files_controller = FilesController::new(
        FileModel::new("/"),
        view_handle.clone(),
        files_repository.clone(),
    );

    let files_controller_clone = files_controller.clone();
    let on_back = {
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
                adapter.set_active_view(ActiveView::Files);
            });

            files_controller_clone.set_back_enabled(file_model_stack.borrow().len() > 1);
        }
    };

    files_controller.on_back(on_back.clone());
    files_controller.on_show_about(on_show_about.clone());
    files_controller.on_add_bookmark({
        let bookmarks_controller = bookmarks_controller.clone();

        move |file_model| {
            // FIXME
            bookmarks_controller.add_bookmark(BookmarkModel::new(
                coop_local::models::BookmarkType::Dir,
                file_model.name().unwrap_or_default(),
                file_model.path(),
            ))
        }
    });

    let games_repository = GamesRepository::new();
    let games_controller = GamesController::new(view_handle.clone(), games_repository.clone());
    games_controller.on_back(on_back.clone());
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
                adapter.set_active_view(ActiveView::Files);
            });

            files_controller_clone.set_back_enabled(file_model_stack.borrow().len() > 1);
        }
    });

    let text_controller = TextController::new(view_handle.clone(), files_repository.clone());
    text_controller.on_back(on_back.clone());
    text_controller.on_show_about(on_show_about.clone());

    let image_controller = ImageController::new(view_handle.clone(), files_repository);
    image_controller.on_back(on_back);
    image_controller.on_show_about(on_show_about);

    let files_controller_clone = files_controller.clone();
    let bookmark_controller_clone = bookmarks_controller.clone();

    let open_internal = {
        let file_model_stack = file_model_stack.clone();
        let view_handle = view_handle.clone();
        let image_controller = image_controller.clone();

        move |file_model: FileModel| {
            if file_model_stack.borrow().last().eq(&Some(&file_model)) {
                return;
            }

            file_model_stack.borrow_mut().push(file_model.clone());
            bookmark_controller_clone.select(file_model.path());

            if file_model.is_dir() {
                if let Ok(is_games) = games_repository.is_games_dir(file_model.path()) {
                    if is_games {
                        games_controller.show_games(file_model);
                        upgrade_adapter(&view_handle, |adapter| {
                            adapter.set_active_view(ActiveView::Games);
                        });
                        return;
                    }
                }
                files_controller_clone.show_files(file_model);
                files_controller_clone.set_back_enabled(file_model_stack.borrow().len() > 1);

                upgrade_adapter(&view_handle, |adapter| {
                    adapter.set_active_view(ActiveView::Files);
                });
            } else if file_model.file_type() == FileType::Image {
                image_controller.show_image(file_model);
                upgrade_adapter(&view_handle, |adapter| {
                    adapter.set_active_view(ActiveView::Image);
                });
            } else if file_model.file_type() == FileType::Text {
                text_controller.show_text(file_model);
                upgrade_adapter(&view_handle, |adapter| {
                    adapter.set_active_view(ActiveView::Text);
                });
            }
        }
    };

    files_controller.on_open_internal(open_internal.clone());
    bookmarks_controller.on_open_internal(open_internal);

    let update_timer = Timer::default();
    update_timer.start(
        TimerMode::Repeated,
        std::time::Duration::from_millis(200),
        {
            let files_controller = files_controller.clone();
            move || {
                files_controller.update();
            }
        },
    );

    main_window.run()?;

    Ok(())
}

fn upgrade_adapter(view_handle: &Weak<MainWindow>, func: impl FnOnce(MainAdapter) + 'static) {
    if let Some(view) = view_handle.upgrade() {
        func(view.global::<MainAdapter>());
    }
}
