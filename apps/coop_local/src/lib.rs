// SPDX-FileCopyrightText: 2023 Florian Blasius <co_sl@tutanota.com>
// SPDX-License-Identifier: GPL-3.0-only

pub mod controllers;
#[cfg(feature = "games")]
pub mod gamepad;

pub mod models;
pub mod proxy_model;
pub mod repositories;
pub mod services;
pub mod ui;
