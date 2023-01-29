// SPDX-FileCopyrightText: 2022 Florian Blasius <co_sl@tutanota.com>
// SPDX-License-Identifier: MIT

fn main() {
    pico_engine_ui::generate_import().unwrap();
    slint_build::compile("ui/game.slint").unwrap();
}
