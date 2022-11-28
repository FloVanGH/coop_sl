// SPDX-FileCopyrightText: 2022 Florian Blasius <co_sl@tutanota.com>
// SPDX-License-Identifier: MIT

/*!
# slint_orbclient

[Slint](https://slint-ui.com/) platform support based on [OrbClient](https://gitlab.redox-os.org/redox-os/orbclient). Can be used to run a `Slint` application onf [Redox](https://redox-os.org/).
*/

mod color;
mod event_reader;
mod platform;

pub use self::color::*;
pub use self::event_reader::*;
pub use self::platform::*;

/// Initializes the platform.
pub fn init() {
    slint::platform::set_platform(Box::new(OrbClientPlatform::new())).unwrap();
}

/// Initializes the platform with the given config
pub fn init_config(config: Config) {
    slint::platform::set_platform(Box::new(OrbClientPlatform::from(config))).unwrap();
}
