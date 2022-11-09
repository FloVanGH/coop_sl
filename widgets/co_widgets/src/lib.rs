// SPDX-FileCopyrightText: 2022 Florian Blasius <flovanpt@posteo.de>
// SPDX-License-Identifier: MIT

/*!

# co_widgets

This crate provides a custom set of widgets for [Slint](https://slint-ui.com) with a custom design (light and dark).

## Component overview

* [Components](docs::components)
* [Layouts](docs::layouts)
* [Widgets](docs::widgets)

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

`co_widgets::generate_import().unwrap();`

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
*/

use std::{env, fs, io, io::Write, path::Path};

/// Generates a import file for the widget library on the given path e.g. `my_project/my_ui/_my_imports`.
pub fn generate_import_with_custom_ui_path<P>(ui_path: P) -> io::Result<()>
where
    P: AsRef<Path>,
{
    let import_path = ui_path.as_ref().to_path_buf();
    let ui_lib_name = env!("UI_LIB_NAME");
    let ui_lib_path = env!("UI_LIB_PATH");
    let ui_lib_file = env!("UI_LIB_FILE");

    let import_file_content = fs::read_to_string(ui_lib_file)
        .map(|c| c.replace("from \"", format!("from \"{}/", ui_lib_path).as_str()))?;

    if !import_path.exists() {
        fs::create_dir_all(import_path.clone())?;
    }

    let mut import_file = fs::File::create(import_path.join(format!("{}.slint", ui_lib_name)))?;

    import_file.write_all(import_file_content.as_bytes())
}

/// Generates a import file for the widget library on a default ui path `my_project/ui/_imports`.
pub fn generate_import() -> io::Result<()> {
    generate_import_with_custom_ui_path(env::current_dir()?.join("ui/_imports"))
}

#[cfg(doc)]
pub mod docs;
