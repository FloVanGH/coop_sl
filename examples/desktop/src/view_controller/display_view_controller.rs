// SPDX-FileCopyrightText: 2022 Florian Blasius <co_sl@tutanota.com>
// SPDX-License-Identifier: MIT

use crate::*;

use std::rc::Rc;
use std::thread;

use slint::{Model, SharedString, VecModel, Weak};

use coop_protocol::*;

pub struct DisplayViewController {
    display_loop: thread::JoinHandle<()>,
}

impl DisplayViewController {
    pub fn new(desktop: &Desktop) -> Self {
        let window_models = Rc::new(VecModel::default());

        desktop
            .global::<DisplayViewAdapter>()
            .set_window_models(window_models.into());

        let display_loop = thread::spawn({
            let desktop = desktop.as_weak();

            move || {
                display_loop(desktop).expect("Display loop error.");
            }
        });

        Self { display_loop }
    }

    pub fn join(self) -> std::thread::Result<()> {
        self.display_loop.join()
    }
}

fn display_loop(desktop: Weak<Desktop>) -> tokio::io::Result<()> {
    let mut server = coop_server::Server::new();
    let proxy = server.slint_proxy();

    desktop
        .upgrade_in_event_loop({
            let proxy = proxy.clone();

            move |desktop| {
                desktop.window().on_close_requested({
                    let proxy = proxy.clone();
                    move || {
                        proxy.close();
                        slint::CloseRequestResponse::HideWindow
                    }
                });
                desktop.global::<DisplayViewAdapter>().on_open({
                    let proxy = proxy.clone();
                    let desktop = desktop.as_weak();

                    move |path| {
                        let proxy = proxy.clone();
                        desktop
                            .upgrade_in_event_loop(move |desktop| {
                                let models =
                                    desktop.global::<DisplayViewAdapter>().get_window_models();
                                let desktop = desktop.as_weak();
                                for i in 0..models.row_count() {
                                    let model = models.row_data(i).unwrap();
                                    if model.path.eq(&path) {
                                        bring_to_front(desktop, model.key);
                                        return;
                                    }
                                }

                                proxy.open_client(path);
                            })
                            .expect("Cannot update in event loop");
                    }
                });

                desktop.global::<DisplayViewAdapter>().on_close_all({
                    let proxy = proxy.clone();
                    move || {
                        println!("start close all");
                        proxy.close();
                    }
                });

                desktop.global::<DisplayViewAdapter>().on_pointer_event({
                    let proxy = proxy.clone();
                    move |key, x, y, event| {
                        proxy.send_pointer_event(key, x, y, event);
                    }
                });

                desktop
                    .global::<DisplayViewAdapter>()
                    .on_key_pressed_event({
                        let proxy = proxy.clone();
                        move |key, event| {
                            proxy.send_key_pressed_event(key, event);
                        }
                    });

                let proxy = proxy.clone();
                desktop
                    .global::<DisplayViewAdapter>()
                    .on_key_released_event({
                        let proxy = proxy.clone();
                        move |key, event| {
                            proxy.send_key_released_event(key, event);
                        }
                    });

                let proxy = proxy.clone();
                desktop.global::<DisplayViewAdapter>().on_close({
                    let proxy = proxy.clone();
                    move |key| {
                        proxy.close_client(key);
                    }
                });

                desktop.global::<DisplayViewAdapter>().on_bring_to_front({
                    let desktop = desktop.as_weak();
                    move |key| {
                        let desktop = desktop.clone();
                        bring_to_front(desktop, key);
                    }
                });
            }
        })
        .expect("Cannot upgrade in desktop event loop");

    'server: loop {
        if let Some(response) = server.client_response() {
            match response.message {
                ClientResponseMessage::Created(c) => {
                    desktop
                        .upgrade_in_event_loop(move |desktop| {
                            let client = WindowModel {
                                key: response.key.into(),
                                x: 10.0.into(),
                                y: 10.0.into(),
                                width: c.width.into(),
                                height: c.height.into(),
                                title: c.title.into(),
                                buffer: slint::Image::default(),
                                path: c.path.into(),
                            };

                            if let Some(clients) = desktop
                                .global::<DisplayViewAdapter>()
                                .get_window_models()
                                .as_any()
                                .downcast_ref::<VecModel<WindowModel>>()
                            {
                                clients.push(client);
                            }
                        })
                        .expect("Cannot add client to list of window clients");
                }
                ClientResponseMessage::Draw(d) => {
                    desktop
                        .upgrade_in_event_loop(move |desktop| {
                            let clients =
                                desktop.global::<DisplayViewAdapter>().get_window_models();

                            for row in 0..clients.row_count() {
                                if let Some(mut c) = clients.row_data(row) {
                                    if c.key.to_string().eq(&response.key) {
                                        let spb = slint::SharedPixelBuffer::<
                                                            slint::Rgba8Pixel,
                                                        >::clone_from_slice(
                                                            &d.buffer, d.width as u32, 400
                                                        );

                                        c.buffer = slint::Image::from_rgba8(spb);
                                        clients.set_row_data(row, c);
                                    }
                                }
                            }
                        })
                        .expect("Cannot draw.");
                }
                ClientResponseMessage::Closed => {
                    desktop
                        .upgrade_in_event_loop(move |desktop| {
                            let clients =
                                desktop.global::<DisplayViewAdapter>().get_window_models();

                            for row in 0..clients.row_count() {
                                if let Some(c) = clients.row_data(row) {
                                    if c.key.to_string().eq(&response.key) {
                                        if let Some(clients) = desktop
                                            .global::<DisplayViewAdapter>()
                                            .get_window_models()
                                            .as_any()
                                            .downcast_ref::<VecModel<WindowModel>>()
                                        {
                                            clients.remove(row);
                                        }
                                    }
                                }
                            }
                        })
                        .expect("Cannot draw.");
                }
                ClientResponseMessage::AllClosed => {
                    break 'server;
                }
            }
        }
    }

    Ok(())
}

fn bring_to_front(desktop: Weak<Desktop>, key: SharedString) {
    desktop
        .upgrade_in_event_loop(move |desktop| {
            for i in 0..desktop
                .global::<DisplayViewAdapter>()
                .get_window_models()
                .row_count()
            {
                if desktop
                    .global::<DisplayViewAdapter>()
                    .get_window_models()
                    .row_data(i)
                    .unwrap()
                    .key
                    .eq(&key)
                {
                    let window = desktop
                        .global::<DisplayViewAdapter>()
                        .get_window_models()
                        .row_data(i)
                        .unwrap();

                    desktop
                        .global::<DisplayViewAdapter>()
                        .get_window_models()
                        .as_any()
                        .downcast_ref::<VecModel<WindowModel>>()
                        .unwrap()
                        .remove(i);

                    desktop
                        .global::<DisplayViewAdapter>()
                        .get_window_models()
                        .as_any()
                        .downcast_ref::<VecModel<WindowModel>>()
                        .unwrap()
                        .push(window);
                }
            }
        })
        .expect("Cannot replace window order.");
}
