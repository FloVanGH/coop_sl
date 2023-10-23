// SPDX-FileCopyrightText: 2023 Florian Blasius <co_sl@tutanota.com>
// SPDX-License-Identifier: GPL-3.0-only

const MAIN_WINDOW: &str = "ui/main-window.slint";

fn main() {
    slint_build::compile_with_config(
        MAIN_WINDOW,
        slint_build::CompilerConfiguration::new().with_library_paths(coop::import_paths()),
    )
    .unwrap();
}
