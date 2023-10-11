// SPDX-FileCopyrightText: 2022 Florian Blasius <co_sl@tutanota.com>
// SPDX-License-Identifier: MIT

use std::{env, fs, io, io::Write, path::Path};

/// Generates a import file for the widget library on the given path e.g. `my_project/my_ui/_my_imports`.
pub fn generate_import_with_custom_ui_path<P>(ui_path: P) -> io::Result<()>
where
    P: AsRef<Path>,
{
    let main_import_path = ui_path.as_ref().to_path_buf();

    let ui_lib_name = env!("UI_LIB_NAME");
    let ui_lib_path = env!("UI_LIB_PATH");
    let ui_lib_file = env!("UI_LIB_FILE");

    let sub_import_path = main_import_path.join(ui_lib_name);

    if !sub_import_path.exists() {
        fs::create_dir_all(sub_import_path.clone())?;
    }

    let import_file_content = fs::read_to_string(ui_lib_file)
        .map(|c| c.replace("from \"", format!("from \"{ui_lib_name}/").as_str()))?;

    let mut import_file = fs::File::create(main_import_path.join(format!("{ui_lib_name}.slint")))?;

    create_import_file(
        &Path::new(ui_lib_path).join("building-blocks.slint"),
        ui_lib_path,
        &sub_import_path,
    )?;
    create_import_file(
        &Path::new(ui_lib_path).join("components.slint"),
        ui_lib_path,
        &sub_import_path,
    )?;
    create_import_file(
        &Path::new(ui_lib_path).join("enums.slint"),
        ui_lib_path,
        &sub_import_path,
    )?;
    create_import_file(
        &Path::new(ui_lib_path).join("layouts.slint"),
        ui_lib_path,
        &sub_import_path,
    )?;
    create_import_file(
        &Path::new(ui_lib_path).join("keyboard.slint"),
        ui_lib_path,
        &sub_import_path,
    )?;
    create_import_file(
        &Path::new(ui_lib_path).join("structs.slint"),
        ui_lib_path,
        &sub_import_path,
    )?;
    create_import_file(
        &Path::new(ui_lib_path).join("styling.slint"),
        ui_lib_path,
        &sub_import_path,
    )?;
    create_import_file(
        &Path::new(ui_lib_path).join("widgets.slint"),
        ui_lib_path,
        &sub_import_path,
    )?;

    import_file.write_all(import_file_content.as_bytes())
}

/// Generates a import file for the widget library on a default ui path `my_project/ui/_imports`.
pub fn generate_import() -> io::Result<()> {
    generate_import_with_custom_ui_path(env::current_dir()?.join("ui/_imports"))
}

fn create_import_file(
    source_path: &Path,
    import_path: &str,
    sub_import_path: &Path,
) -> io::Result<()> {
    // let source_path = Path::new(source);

    let source_content = fs::read_to_string(source_path)
        .map(|c| c.replace("from \"", format!("from \"{import_path}/").as_str()))?;

    if let Some(file_name) = source_path.file_name() {
        let mut import_file = fs::File::create(sub_import_path.join(file_name))?;
        import_file.write_all(source_content.as_bytes())?;
    }

    Ok(())
}
