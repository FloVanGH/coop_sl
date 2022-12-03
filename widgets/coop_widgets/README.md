<!--
SPDX-FileCopyrightText: 2022 Florian Blasius <co_sl@tutanota.com>
SPDX-License-Identifier: MIT
-->

# coop_widgets

The goal with `coop_widgets` is to provide a custom widget and components library for [Slint](https://slint-ui.com/) with a simple, consistence and clean design. The second goal is to serve as an example how to implement a widget library with Slint based on a custom design system. The theme is available in a light and a dark variant.

What the `coop` in `coop_widgets` stands for: cooperation.

<a href="https://codeberg.org/flovansl/coop_sl">
    <img alt="Get it on Codeberg" src="https://get-it-on.codeberg.org/get-it-on-blue-on-white.png" height="40">
</a>

[![MIT licensed](https://img.shields.io/badge/license-MIT-blue.svg)](../../LICENSES/MIT.txt)
[![docs](https://img.shields.io/badge/docs-latest-orange.svg)](https://flovansl.codeberg.page/snapshots/coop_sl/docs/coop_widgets/)

## live preview

Check the [Online wasm demo](https://flovansl.codeberg.page/snapshots/widgets/).


## how to use

1. Add `coop_widgets` as build dependency to your `Cargo.toml`:

```toml
[dependencies]
slint = "0.3.2"

[build-dependencies]
slint-build = "0.3.2"
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

export MyApp := CoopWindow {
    preferred-width: 600px;
    preferred-height: 400px;
    title: "MyApp";

    Button {
        text: "Click me";
    }
}
```

## structure

* widgets: ...


## theming

All theme resources of `coop_widgets` like brushes, sizes, spacings and typography can be accessed by using the  `coop` global.

### brush example

```
import { coop } from "_imports/coop_widgets.slint";

MyRect := Rectangle {
    background: coop.theme.brushes.surface;
}
```

### typo example

```
import { coop } from "_imports/coop_widgets.slint";

MyText := Text {
   font_size: coop.theme.typo.large_label.size;
   font_weight: coop.theme.typo.large_label.weight;
}
```

### spaces

```
import { coop } from "_imports/coop_widgets.slint";

MyLayout := HorizontalLayout {
   spacing: coop.theme.spaces.small;
   padding: coop.theme.spaces.medium;
}
```

## new to slint?

Best to start with these sources:

* getting start: https://slint-ui.com/#tryout
* Slint docs (*Slint lang docs included*) : https://docs.rs/slint/latest/slint/
* Slint examples: https://github.com/slint-ui/slint/tree/master/examples
* Slint chat: https://chat.slint-ui.com/

## assets

* Material Icons
    * `coop_widgets` uses [Material Icons](https://fonts.google.com/icons) as icon font. The icons are licensed under Apache 2.0.


## license

* `coop_widgets` is available under [MIT license](LICENSE-MIT)
* `Slint` is available under either a [commercial license](https://github.com/slint-ui/slint/blob/master/LICENSES/LicenseRef-Slint-commercial.md)
or [GNU GPLv3](https://github.com/slint-ui/slint/blob/master/LICENSES/GPL-3.0-only.txt).
