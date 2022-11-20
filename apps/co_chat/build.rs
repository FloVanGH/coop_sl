// SPDX-FileCopyrightText: 2022 Florian Blasius <co_sl@tutanota.com>
// SPDX-License-Identifier: GPL-3.0-only

fn main() {
    co_widgets::generate_import().unwrap();
    slint_build::compile("ui/app.slint").unwrap();
}
