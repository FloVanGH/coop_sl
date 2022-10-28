#[cfg(features = "pico")]
fn main() {
    let config = slint_build::CompilerConfiguration::new()
        .embed_resources(slint_build::EmbedResourcesKind::EmbedForSoftwareRenderer);
    slint_build::compile_with_config("ui/widgets.slint", config).unwrap();
    slint_build::print_rustc_flags().unwrap();
}

#[cfg(not(features = "pico"))]
fn main() {
    slint_build::compile("ui/widgets.slint").unwrap();
}
