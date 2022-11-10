// SPDX-FileCopyrightText: 2022 Florian Blasius <co_sl@tutanota.com>
// SPDX-License-Identifier: MIT

#![cfg(doc)]

/*!
    This is only a helper to generate the documentation for the Slint components.
*/

pub mod components {
    #![doc = include_str!("../ui/components/_components.md")]
}

pub mod layouts {
    #![doc = include_str!("../ui/layouts/_layouts.md")]
}

pub mod widgets {
    #![doc = include_str!("../ui/widgets/_widgets.md")]
}
