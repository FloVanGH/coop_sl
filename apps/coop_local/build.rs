// SPDX-FileCopyrightText: 2023 Florian Blasius <co_sl@tutanota.com>
// SPDX-License-Identifier: GPL-3.0-only

const MAIN_WINDOW: &str = "ui/main-window.slint";

fn main() {
    coop_widgets::generate_import().unwrap();
    slint_build::compile(MAIN_WINDOW).unwrap();
}
