// SPDX-FileCopyrightText: 2022 Florian Blasius <co_sl@tutanota.com>
// SPDX-License-Identifier: MIT

use std::collections::HashMap;

const APP: &str = "ui/app.slint";

#[cfg(feature = "default")]
fn main() {
    let mut import_paths = HashMap::new();
    import_paths.extend(coop::import_paths());
    import_paths.extend(book_flip::import_paths());

    slint_build::compile_with_config(
        APP,
        slint_build::CompilerConfiguration::new().with_library_paths(import_paths),
    )
    .unwrap();
}

#[cfg(any(
    feature = "mcu-board-support",
    target_os = "redox",
    feature = "slint_coop"
))]
fn main() {
    let mut import_paths = HashMap::new();
    import_paths.extend(coop::import_paths().into_iter());
    import_paths.extend(book_flip::import_paths().into_iter());

    let config = slint_build::CompilerConfiguration::new()
        .embed_resources(slint_build::EmbedResourcesKind::EmbedForSoftwareRenderer)
        .with_library_paths(import_paths);
    slint_build::compile_with_config(APP, config).unwrap();
    slint_build::print_rustc_flags().unwrap();
}
