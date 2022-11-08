# co_widgets

The goal with `co_widgets` is to provide a custom widget and components library for [Slint](https://slint-ui.com/) with a simple, consistence and clean design. The second goal is to serve as an example how to implement a widget library with Slint based on a custom design system. The theme is available in a light and a dark variant.

What the `co` in `co_widgets` stands for: cooperation.

<a href="https://codeberg.org/flovansl/co_sl">
    <img alt="Get it on Codeberg" src="https://get-it-on.codeberg.org/get-it-on-blue-on-white.png" height="40">
</a>

[![MIT licensed](https://img.shields.io/badge/license-MIT-blue.svg)](../../LICENSE-MIT)

## how to use

1. Add `co_widgets` as build dependency to your `Cargo.toml`:

```toml
[dependencies]
slint = "0.3.1"

[build-dependencies]
slint-build = "0.3.1"
co_widgets = { ... }
```

2. Call `co_widgets::generate_import()` from your `build.rs` file. It will generate an import file `../$MY_PROJECT_PATH/ui/_imports/co_widgets.slint`:

```rust
fn main() {
    co_widgets::generate_import().unwrap();
    slint_build::compile("ui/my_app.slint").unwrap();
}
```

3. Add an import to your slint file:

```slint,no-preview
import { CoWindow, Button } from "_imports/co_widgets.slint";

export MyApp := CoWindow {
    preferred-width: 600px;
    preferred-height: 400px;
    title: "MyApp";

    Button {
        text: "Click me";
    }
}
```

## example

[Online wasm demo](https://flovansl.codeberg.page/snapshots/widgets/)


## structure

* widgets: ...


## theming

All theme resources of `co_widgets` like brushes, sizes, spacings and typography can be accessed by using the  `co` global.

### brush example

```
import { co } from "_imports/co_widgets.slint";

MyRect := Rectangle {
    background: co.brushes.surface;
}
```

### typo example

```
import { co } from "_imports/co_widgets.slint";

MyText := Text {
   font_size: co.typo.label_large.size;
   font_family: co.typo.label_large.family;
   font_weight: co.typo.label_large.weight;
}
```

### spaces

```
import { co } from "_imports/co_widgets.slint";

MyLayout := HorizontalLayout {
   spacing: co.spaces.small;
   padding: co.spaces.medium;
}
```

## new to slint?

Best to start with these sources:

* getting start: https://slint-ui.com/#tryout
* Slint language docs: https://github.com/slint-ui/slint/tree/master/docs
* Slint examples: https://github.com/slint-ui/slint/tree/master/examples
* Slint chat: https://chat.slint-ui.com/

## assets

* Roboto
    * `co_widgets` uses [Roboto](https://fonts.google.com/specimen/Roboto) as default font set. `Roboto` are licensed under [Apache 2.0](co_widgets/assets/fonts/Roboto-LICENSE.txt).
* Material Icons
    * `co_widgets` uses [Material Icons](https://fonts.google.com/icons) as icon font. The icons are licensed under [Apache 2.0](co_widgets/assets/icons/Material-Icons-LICENSE.txt).


## license

* `co_widgets` is available under [MIT license](LICENSE-MIT)
* `Slint` is available under either a [commercial license](https://github.com/slint-ui/slint/blob/master/LICENSES/LicenseRef-Slint-commercial.md)
or [GNU GPLv3](https://github.com/slint-ui/slint/blob/master/LICENSES/GPL-3.0-only.txt).
