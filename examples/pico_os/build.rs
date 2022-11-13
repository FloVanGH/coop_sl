// SPDX-FileCopyrightText: 2022 Florian Blasius <co_sl@tutanota.com>
// SPDX-License-Identifier: MIT

fn main() {
    co_widgets::generate_import().unwrap();
    slint_build::compile("ui/os.slint").unwrap();
}
