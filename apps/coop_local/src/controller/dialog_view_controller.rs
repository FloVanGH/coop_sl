// SPDX-FileCopyrightText: 2023 Florian Blasius <co_sl@tutanota.com>
// SPDX-License-Identifier: GPL-3.0-only

use crate::ui;
use slint::*;
use std::io;
use tokio::runtime::Builder;
use tokio::sync::mpsc;

pub enum DialogResponse {
    Canceled,
    TextUpdate(String),
}

pub enum DialogViewMessage {
    ShowTextInput {
        title: String,
        default_button_text: String,
        text: String,
        respond_to: mpsc::Sender<DialogResponse>,
    },
    ShowAbout,
}

#[derive(Clone)]
pub struct DialogViewController {
    spawn: mpsc::Sender<DialogViewMessage>,
}

impl DialogViewController {
    pub fn new(main_window: &ui::MainWindow) -> io::Result<Self> {
        let rt = Builder::new_current_thread().enable_all().build()?;
        let (send, mut recv) = mpsc::channel(16);
        let controller = Self { spawn: send };

        std::thread::spawn({
            let window_handle = main_window.as_weak();

            move || {
                rt.block_on(async move {
                    while let Some(message) = recv.recv().await {
                        tokio::spawn(handle_message(message, window_handle.clone()));
                    }
                });
            }
        });

        Ok(controller)
    }

    pub async fn spawn_message(&self, message: DialogViewMessage) {
        let _ = self.spawn.send(message).await;
    }
}

async fn handle_message(message: DialogViewMessage, window_handle: Weak<ui::MainWindow>) {
    match message {
        DialogViewMessage::ShowTextInput {
            title,
            default_button_text,
            text,
            respond_to,
        } => {
            let _ = window_handle.upgrade_in_event_loop(move |main_window| {
                main_window.global::<ui::DialogViewAdapter>().on_cancel({
                    let window_handle = main_window.as_weak();
                    let respond_to = respond_to.clone();

                    move || {
                        let _ = respond_to.blocking_send(DialogResponse::Canceled);
                        if let Some(main_window) = window_handle.upgrade() {
                            main_window
                                .global::<ui::DialogViewAdapter>()
                                .set_ti_visible(false);
                        }
                    }
                });

                main_window
                    .global::<ui::DialogViewAdapter>()
                    .on_ti_default({
                        let window_handle = main_window.as_weak();

                        move |text| {
                            let _ =
                                respond_to.blocking_send(DialogResponse::TextUpdate(text.into()));
                            if let Some(main_window) = window_handle.upgrade() {
                                main_window
                                    .global::<ui::DialogViewAdapter>()
                                    .set_ti_visible(false);
                            }
                        }
                    });
            });

            tokio::spawn(show_text_input(
                title,
                default_button_text,
                text,
                window_handle,
            ));
        }
        DialogViewMessage::ShowAbout => {
            tokio::spawn(show_about(window_handle));
        }
    }
}

async fn show_text_input(
    title: String,
    default_button_text: String,
    text: String,
    window_handle: Weak<ui::MainWindow>,
) {
    let _ = window_handle.upgrade_in_event_loop(move |main_window| {
        main_window
            .global::<ui::DialogViewAdapter>()
            .set_title(title.into());
        main_window
            .global::<ui::DialogViewAdapter>()
            .set_default_button_text(default_button_text.into());
        main_window
            .global::<ui::DialogViewAdapter>()
            .set_ti_text(text.into());
        main_window
            .global::<ui::DialogViewAdapter>()
            .set_ti_visible(true);
    });
}

async fn show_about(window_handle: Weak<ui::MainWindow>) {
    let _ = window_handle.upgrade_in_event_loop(move |main_window| {
        main_window.global::<ui::DialogViewAdapter>().on_cancel({
            let window_handle = main_window.as_weak();

            move || {
                if let Some(main_window) = window_handle.upgrade() {
                    main_window
                        .global::<ui::DialogViewAdapter>()
                        .set_about_visible(false);
                }
            }
        });
        main_window
            .global::<ui::DialogViewAdapter>()
            .set_about_visible(true);
    });
}
