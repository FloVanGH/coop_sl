// SPDX-FileCopyrightText: 2022 Florian Blasius <co_sl@tutanota.com>
// SPDX-License-Identifier: MIT

#![no_std]

extern crate alloc;

mod color;
mod events;
mod platform;

pub use self::color::*;
pub use self::events::*;
pub use self::platform::*;
