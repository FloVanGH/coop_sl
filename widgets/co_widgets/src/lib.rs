#![cfg_attr(not(feature = "std"), no_std)]

use std::{env, fs, io, io::Write, path::Path};

/// Generates an include file for the `co_widgets` slint widget library.
/// Generated file is located on "slint_includes/co_widgets.slint"
pub fn create_include() -> io::Result<()> {
    let lib_name = env!("CO_WIDGETS_LIB_NAME");
    let include_path = env!("CO_WIDGETS_INCLUDE_PATH");
    let include_file = env!("CO_WIDGETS_INCLUDE_FILE");
    let include_dir = env::current_dir().unwrap().join("_slint_includes");

    let export_include_file_content =
        fs::read_to_string(include_file).and_then(|c| Ok(c.replace(lib_name, include_path)))?;

    if !include_dir.exists() {
        fs::create_dir(include_dir.clone())?;
    }

    let mut export_include_file =
        fs::File::create(include_dir.join(Path::new(include_file).file_name().ok_or(
            io::Error::new(io::ErrorKind::Other, "Cannot read include file name"),
        )?))?;

    export_include_file.write_all(export_include_file_content.as_bytes())
}
