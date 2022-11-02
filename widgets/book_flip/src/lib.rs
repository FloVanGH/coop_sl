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
