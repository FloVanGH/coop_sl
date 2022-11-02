#[cfg(not(feature = "mcu-board-support"))]
fn main() {
    co_widgets::create_include().unwrap();
    slint_build::compile("ui/widgets.slint").unwrap();
}

#[cfg(feature = "mcu-board-support")]
fn main() {
    co_widgets::create_include().unwrap();
    let config = slint_build::CompilerConfiguration::new()
        .embed_resources(slint_build::EmbedResourcesKind::EmbedForSoftwareRenderer);
    slint_build::compile_with_config("ui/widgets.slint", config).unwrap();
    slint_build::print_rustc_flags().unwrap();
}
