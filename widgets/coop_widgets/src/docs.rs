// SPDX-FileCopyrightText: 2022 Florian Blasius <co_sl@tutanota.com>
// SPDX-License-Identifier: MIT

#![cfg(doc)]

/*!
    This is only a helper to generate the documentation for the Slint components.
*/

pub mod building_blocks {
    #![doc = include_str!("../ui/building_blocks/_building_blocks.md")]
}

pub mod coop {
    #![doc = include_str!("../ui/coop/_coop.md")]
}

pub mod components {
    #![doc = include_str!("../ui/components/_components.md")]
}

pub mod keyboard {
    #![doc = include_str!("../ui/keyboard/_keyboard.md")]
}

pub mod layouts {
    #![doc = include_str!("../ui/layouts/_layouts.md")]
}

pub mod widgets {
    #![doc = include_str!("../ui/widgets/_widgets.md")]
}
