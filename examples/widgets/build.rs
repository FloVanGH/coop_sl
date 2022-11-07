fn generate_imports() {
    book_flip::generate_import().unwrap();
    co_widgets::generate_import().unwrap();
}

#[cfg(not(feature = "mcu-board-support"))]
const APP: &'static str = "ui/app.slint";

#[cfg(feature = "mcu-board-support")]
const APP: &'static str = "ui/minimize/app.slint";

#[cfg(all(not(feature = "mcu-board-support"), not(feature = "slint_redox")))]
fn main() {
    generate_imports();
    slint_build::compile(APP).unwrap();
}

#[cfg(any(feature = "mcu-board-support", feature = "slint_redox"))]
fn main() {
    generate_imports();
    let config = slint_build::CompilerConfiguration::new()
        .embed_resources(slint_build::EmbedResourcesKind::EmbedForSoftwareRenderer);
    slint_build::compile_with_config(App, config).unwrap();
    slint_build::print_rustc_flags().unwrap();
}
