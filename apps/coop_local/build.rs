// SPDX-FileCopyrightText: 2023 Florian Blasius <co_sl@tutanota.com>
// SPDX-License-Identifier: GPL-3.0-only

use slint_build::CompilerConfiguration;

const MAIN_WINDOW: &str = "ui/main-window.slint";

fn main() {
    coop_widgets::generate_import().unwrap();
    slint_build::compile_with_config(
        MAIN_WINDOW,
        CompilerConfiguration::new().with_style("fluent".into()),
    )
    .unwrap();
}
