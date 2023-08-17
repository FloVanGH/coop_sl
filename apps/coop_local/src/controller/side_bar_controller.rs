// SPDX-FileCopyrightText: 2023 Florian Blasius <co_sl@tutanota.com>
// SPDX-License-Identifier: GPL-3.0-only

use crate::model::{BookmarkModel, BookmarkType, FileModel};
use crate::proxy_models::BookmarksProxyModel;
use crate::{repository, ui};
use slint::*;
use std::io;
use std::rc::Rc;
use tokio::runtime::Builder;
use tokio::sync::{mpsc, oneshot};

mod context_menu;

#[derive(Clone)]
pub struct SideBarController {
    spawn: mpsc::Sender<SideBarMessage>,
}

impl SideBarController {
    pub fn new<B>(main_window: &ui::MainWindow, bookmarks_repository: B) -> io::Result<Self>
    where
        B: repository::traits::BookMarksRepository + Clone + std::marker::Send + 'static,
    {
        let rt = Builder::new_current_thread().enable_all().build()?;
        let (send, mut recv) = mpsc::channel(16);

        let controller = Self { spawn: send };

        init(&controller, main_window, bookmarks_repository.clone());

        std::thread::spawn({
            let window_handle = main_window.as_weak();

            move || {
                rt.block_on(async move {
                    while let Some(message) = recv.recv().await {
                        tokio::spawn(handle_message(
                            message,
                            window_handle.clone(),
                            bookmarks_repository.clone(),
                        ));
                    }
                });
            }
        });

        Ok(controller)
    }

    pub fn spawn_message(&self, message: SideBarMessage) {
        let _ = self.spawn.blocking_send(message);
    }
}

pub enum SideBarMessage {
    AddBookmark { path: String },
    Open { parent: usize, bookmark: usize },
    RemoveBookmark { bookmark: usize },
}

async fn handle_message<B>(
    message: SideBarMessage,
    window_handle: Weak<ui::MainWindow>,
    bookmarks_repository: B,
) where
    B: repository::traits::BookMarksRepository + Clone + std::marker::Send + 'static,
{
    match message {
        SideBarMessage::AddBookmark { path } => {
            tokio::spawn(on_add_bookmark(path, window_handle, bookmarks_repository));
        }
        SideBarMessage::Open { parent, bookmark } => {
            tokio::spawn(open(parent, bookmark, window_handle));
        }
        SideBarMessage::RemoveBookmark { bookmark } => {
            tokio::spawn(remove_bookmark(
                bookmark,
                window_handle,
                bookmarks_repository,
            ));
        }
    }
}

fn init<B>(controller: &SideBarController, main_window: &ui::MainWindow, bookmarks_repository: B)
where
    B: repository::traits::BookMarksRepository + Clone + std::marker::Send + 'static,
{
    let adapter = main_window.global::<ui::SideBarAdapter>();
    let window_handle = main_window.as_weak();

    adapter.set_bookmarks(VecModel::from_slice(&[
        ui::GroupListViewItem {
            text: "Bookmarks".into(),
            items: Rc::new(BookmarksProxyModel::new(
                bookmarks_repository.bookmarks(),
                window_handle.clone(),
            ))
            .into(),
        },
        ui::GroupListViewItem {
            text: "Locations".into(),
            items: Rc::new(BookmarksProxyModel::new(
                bookmarks_repository.locations(),
                window_handle.clone(),
            ))
            .into(),
        },
    ]));

    adapter.on_add_bookmark({
        let controller = controller.clone();

        move |path| controller.spawn_message(SideBarMessage::AddBookmark { path: path.into() })
    });

    adapter.on_get_context_menu({
        move |parent, _bookmark: i32| context_menu::get_context_menu(parent as usize)
    });

    adapter.on_open({
        let controller = controller.clone();
        move |parent, bookmark| {
            controller.spawn_message(SideBarMessage::Open {
                parent: parent as usize,
                bookmark: bookmark as usize,
            });
        }
    });

    adapter.set_current_bookmark((-1, -1));

    context_menu::on_context_menu_action(controller.clone(), main_window);
}

async fn on_add_bookmark<B>(
    path: String,
    window_handle: Weak<ui::MainWindow>,
    bookmarks_repository: B,
) where
    B: repository::traits::BookMarksRepository + Clone + std::marker::Send + 'static,
{
    let (tx, rx) = oneshot::channel();
    let _ = window_handle.upgrade_in_event_loop({
        let path = path.clone();

        move |main_window| {
            let _ = tx.send(
                main_window
                    .global::<ui::GamesAdapter>()
                    .invoke_is_games_dir(path.into()),
            );
        }
    });

    let bookmark_type = if let Ok(is_games_dir) = rx.await {
        if is_games_dir {
            BookmarkType::Game
        } else {
            BookmarkType::Dir
        }
    } else {
        BookmarkType::Dir
    };

    let bookmark = BookmarkModel::new(
        bookmark_type,
        FileModel::new(path.as_str()).name().unwrap_or_default(),
        path,
    );

    if bookmarks_repository.add_bookmark(&bookmark).is_err() {
        return;
    }

    upgrade_adapter(window_handle, |adapter| {
        if let Some(bookmarks) = adapter
            .get_bookmarks()
            .as_any()
            .downcast_ref::<VecModel<ui::GroupListViewItem>>()
        {
            if let Some(bookmarks) = bookmarks.row_data(0) {
                if let Some(bookmarks) = bookmarks
                    .items
                    .as_any()
                    .downcast_ref::<BookmarksProxyModel>()
                {
                    bookmarks.push(bookmark);
                }
            }
        }
    });
}

async fn open(parent: usize, bookmark: usize, window_handle: Weak<ui::MainWindow>) {
    let _ = window_handle.upgrade_in_event_loop(move |main_window| {
        // FIXME: select when the current bookmark is active on files
        main_window
            .global::<ui::SideBarAdapter>()
            .set_current_bookmark((-1, -1));

        if let Some(bookmarks) = main_window
            .global::<ui::SideBarAdapter>()
            .get_bookmarks()
            .row_data(parent)
        {
            if let Some(bookmarks) = bookmarks
                .items
                .as_any()
                .downcast_ref::<BookmarksProxyModel>()
            {
                if let Some(bookmark) = bookmarks.row_data_as_bookmark(bookmark) {
                    main_window
                        .global::<ui::FilesAdapter>()
                        .invoke_show(bookmark.path().into());
                }
            }
        }
    });
}

async fn remove_bookmark<B>(
    bookmark: usize,
    window_handle: Weak<ui::MainWindow>,
    bookmarks_repository: B,
) where
    B: repository::traits::BookMarksRepository + Clone + std::marker::Send + 'static,
{
    if let Ok(bookmark) = bookmarks_repository.remove_bookmark(bookmark) {
        upgrade_adapter(window_handle, move |adapter| {
            if let Some(bookmarks) = adapter.get_bookmarks().row_data(0) {
                if let Some(bookmarks) = bookmarks
                    .items
                    .as_any()
                    .downcast_ref::<BookmarksProxyModel>()
                {
                    bookmarks.remove(bookmark);
                }
            }
        });
    }
}

// helper

fn upgrade_adapter(
    window_handle: Weak<ui::MainWindow>,
    func: impl FnOnce(ui::SideBarAdapter) + Send + 'static,
) {
    let _ = window_handle
        .upgrade_in_event_loop(move |main_window| func(main_window.global::<ui::SideBarAdapter>()));
}
