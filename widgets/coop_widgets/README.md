<!--
SPDX-FileCopyrightText: 2022 Florian Blasius <co_sl@tutanota.com>
SPDX-License-Identifier: MIT
-->

# coop_widgets

`coop_widgets` is a custom widget and component library for [Slint](https://slint.dev/) with a custom simple, consistence and clean design.

What the `coop` in `coop_widgets` stands for: cooperation.

<a href="https://codeberg.org/flovansl/coop_sl">
    <img alt="Get it on Codeberg" src="https://get-it-on.codeberg.org/get-it-on-blue-on-white.png" height="60">
</a>
<a href=https://slint.dev">
    <img alt="#MadeWithSlint" src="https://raw.githubusercontent.com/slint-ui/slint/master/logo//MadeWithSlint-logo-light.svg" height="60">
</a>

[![MIT licensed](https://img.shields.io/badge/license-MIT-blue.svg)](../../LICENSES/MIT.txt)
[![Rust docs](https://img.shields.io/badge/docs-rust-orange.svg)](https://flovansl.codeberg.page/coop_sl/snapshots/main/docs/rust/coop_widgets/)
[![Slint docs](https://img.shields.io/badge/docs-slint-blue.svg)](https://flovansl.codeberg.page/coop_sl/snapshots/main/docs/slint/coop_widgets/)


## live preview

|[![Screenshot of the gallery](https://codeberg.org/flovansl/pages/attachments/2501a785-2b21-40d8-91c7-85fee14f0045 "gallery")](https://flovansl.codeberg.page/coop_sl/snapshots/main/demos/gallery/) |


## how to use

1. Add `coop_widgets` as build dependency to your `Cargo.toml`:

```toml
[dependencies]
slint = { version = "1.2.2" }

[build-dependencies]
slint-build = { version = "1.2.2" }
coop_widgets = { git = "https://codeberg.org/flovansl/coop_sl" }
```

2. Call `coop_widgets::generate_import()` from your `build.rs` file. It will generate an import file `../$MY_PROJECT_PATH/ui/_imports/coop_widgets.slint`:

```rust
fn main() {
    coop_widgets::generate_import().unwrap();
    slint_build::compile("ui/my_app.slint").unwrap();
}
```

3. Add an import to your slint file:

```slint,no-preview
import { CoopWindow, Button } from "_imports/coop_widgets.slint";

export component MyApp inherits CoopWindow {
    preferred-width: 600px;
    preferred-height: 400px;
    title: "MyApp";

    Button {
        text: "Click me";
    }
}
```

## new to slint?

Best to start with these sources:

* getting start: https://slint.dev/#tryout
* Slint examples: https://github.com/slint-ui/slint/tree/master/examples
* Slint chat: https://chat.slint-ui.com/

## license

* `coop_widgets` is available under [MIT license](LICENSE-MIT)