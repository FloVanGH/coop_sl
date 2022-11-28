<!--
SPDX-FileCopyrightText: 2022 Florian Blasius <florvanpt@posteo.de>
SPDX-License-Identifier: MIT
-->

# slint_psp

[Slint](https://slint-ui.com/) platform support for PSP.

<a href="https://codeberg.org/flovansl/coop_sl">
    <img alt="Get it on Codeberg" src="https://get-it-on.codeberg.org/get-it-on-blue-on-white.png" height="40">
</a>

[![MIT licensed](https://img.shields.io/badge/license-MIT-blue.svg)](../../LICENSES/MIT.txt)

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
    slint_psp::init();
    HelloWorld::new(true).run();
}
```

## Event (button) mapping from PSP to Slint

* `CtrlButtons::RIGHT` => `Key::Tab` (focus next element)
* `CtrlButtons::LEFT` => `Key::Shift` + `Key::Tab` (focus previous element)
* `CtrlButtons::CROSS` => `Key::Return`
* `CtrlButtons::CIRCLE` => `Key::Escape`
* `CtrlButtons::SQUARE` => `PointerEventButton::Left`
* `CtrlButtons::UP` => `PointerScrolled` up
* `CtrlButtons::DOWN` => `PointerScrolled` down
* Stick moved => `PointerMoved`

## new to slint?

Best to start with these sources:

* getting start: https://slint-ui.com/#tryout
* Slint docs (*Slint lang docs included*): https://docs.rs/slint/latest/slint/
* Slint examples: https://github.com/slint-ui/slint/tree/master/examples
* Slint chat: https://chat.slint-ui.com/

## license

* `slint_psp` is available under [MIT license](../../LICENSES/MIT.txt).
* `Slint` is available under either a [commercial license](https://github.com/slint-ui/slint/blob/master/LICENSES/LicenseRef-Slint-commercial.md)
or [GNU GPLv3](https://github.com/slint-ui/slint/blob/master/LICENSES/GPL-3.0-only.txt).
