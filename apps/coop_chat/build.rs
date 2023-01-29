// SPDX-FileCopyrightText: 2022 Florian Blasius <co_sl@tutanota.com>
// SPDX-License-Identifier: GPL-3.0-only

#[cfg(feature = "default")]
fn main() {
    coop_widgets::generate_import().unwrap();
    slint_build::compile("ui/app.slint").unwrap();
}

#[cfg(any(target_os = "redox", feature = "slint_coop"))]
fn main() {
    coop_widgets::generate_import().unwrap();
    let config = slint_build::CompilerConfiguration::new()
        .embed_resources(slint_build::EmbedResourcesKind::EmbedForSoftwareRenderer);
    slint_build::compile_with_config("ui/app.slint", config).unwrap();
    slint_build::print_rustc_flags().unwrap();
}
