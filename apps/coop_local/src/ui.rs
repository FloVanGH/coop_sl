// SPDX-FileCopyrightText: 2023 Florian Blasius <co_sl@tutanota.com>
// SPDX-License-Identifier: GPL-3.0-only

#[allow(clippy::all)]
mod inner {
    #![allow(dead_code)]
    slint::include_modules!();
}

pub use inner::*;
