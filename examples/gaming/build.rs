// SPDX-FileCopyrightText: 2022 Florian Blasius <flovanpt@posteo.de>
// SPDX-License-Identifier: MIT

#[cfg(feature = "std")]
fn main() {
    slint_build::compile("game/game.slint").unwrap();
}

#[cfg(not(feature = "std"))]
fn main() {
    let config = slint_build::CompilerConfiguration::new()
        .embed_resources(slint_build::EmbedResourcesKind::EmbedForSoftwareRenderer);
    slint_build::compile_with_config("game/game.slint", config).unwrap();
    slint_build::print_rustc_flags().unwrap();
}
