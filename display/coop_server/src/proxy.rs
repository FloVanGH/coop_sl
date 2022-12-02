// SPDX-FileCopyrightText: 2022 Florian Blasius <co_sl@tutanota.com>
// SPDX-License-Identifier: MIT

mod client_proxy;
mod sever_proxy;

pub use client_proxy::*;
pub use sever_proxy::*;

#[cfg(feature = "slint")]
mod slint_proxy;

#[cfg(feature = "slint")]
pub use slint_proxy::*;
