// SPDX-FileCopyrightText: 2022 Florian Blasius <co_sl@tutanota.com>
// SPDX-License-Identifier: MIT

fn main() {
    coop_widgets::generate_import().unwrap();
    slint_build::compile("ui/app.slint").unwrap();
}
