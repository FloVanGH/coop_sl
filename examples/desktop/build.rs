// SPDX-FileCopyrightText: 2022 Florian Blasius <co_sl@tutanota.com>
// SPDX-License-Identifier: MIT

fn main() {
    coop::generate_import().unwrap();
    slint_build::compile("ui/app.slint").unwrap();
}
