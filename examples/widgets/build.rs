fn generate_imports() {
    book_flip::generate_import().unwrap();
    co_widgets::generate_import().unwrap();
}

#[cfg(all(not(feature = "mcu-board-support"), not(feature = "slint_redox")))]
fn main() {
    generate_imports();
    slint_build::compile("ui/widgets.slint").unwrap();
}

#[cfg(any(feature = "mcu-board-support", feature = "slint_redox"))]
fn main() {
    generate_imports();
    let config = slint_build::CompilerConfiguration::new()
        .embed_resources(slint_build::EmbedResourcesKind::EmbedForSoftwareRenderer);
    slint_build::compile_with_config("ui/widgets.slint", config).unwrap();
    slint_build::print_rustc_flags().unwrap();
}
