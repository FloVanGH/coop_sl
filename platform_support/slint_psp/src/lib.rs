// SPDX-FileCopyrightText: 2022 Florian Blasius <co_sl@tutanota.com>
// SPDX-License-Identifier: MIT

/*!
# slint_psp

[[Slint](https://slint-ui.com/) platform support for PSP.

## Example

```rust
slint::slint!{
    HelloWorld := Window {
        Text {
            text: "hello world";
            color: green;
        }
    }
}

fn main() {
#   return; // Don't run a window in an example
    slint_psp::init(true);
    HelloWorld::new().run();
}

## Event (button) mapping from PSP to Slint

* `CtrlButtons::RIGHT` => `Key::Tab` (focus next element)
* `CtrlButtons::LEFT` => `Key::Shift` + `Key::Tab` (focus previous element)
* `CtrlButtons::CROSS` => `Key::Return`
* `CtrlButtons::CIRCLE` => `Key::Escape`
* `CtrlButtons::SQUARE` => `PointerEventButton::Left`
* `CtrlButtons::UP` => `PointerScrolled` up
* `CtrlButtons::DOWN` => `PointerScrolled` down
* Stick moved => `PointerMoved`
*/

#![no_std]

extern crate alloc;

mod color;
mod event_reader;
mod platform;

pub use self::color::*;
pub use self::event_reader::*;
pub use self::platform::*;

/// Initializes the psp `Slint` backend.
///
/// If show_pointer is set to `true` a pointer will display the position of the current pointer.
pub fn init(show_pointer: bool) {
    slint::platform::set_platform(alloc::boxed::Box::new(PspPlatform::new(show_pointer))).unwrap();
}
