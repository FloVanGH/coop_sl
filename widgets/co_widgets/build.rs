// SPDX-FileCopyrightText: 2022 Florian Blasius <co_sl@tutanota.com>
// SPDX-License-Identifier: MIT

use std::{env, fs, io, path::*};

fn main() -> io::Result<()> {
    let ui_lib_name = "co_widgets";
    let ui_lib_path = "ui";
    let ui_lib_file = format!("{}/lib.slint", ui_lib_path);

    let manifest_path = Path::new(
        &env::var_os("CARGO_MANIFEST_DIR")
            .ok_or_else(|| io::Error::new(io::ErrorKind::Other, "Cannot read output path"))?,
    )
    .to_path_buf();

    rerun_if_changed(ui_lib_path)?;

    println!(
        "cargo:rustc-env=UI_LIB_PATH={}",
        manifest_path.join(ui_lib_path).as_path().display()
    );
    println!(
        "cargo:rustc-env=UI_LIB_FILE={}",
        manifest_path.join(ui_lib_file).as_path().display()
    );

    println!("cargo:rustc-env=UI_LIB_NAME={}", ui_lib_name);

    Ok(())
}

fn rerun_if_changed<P>(path: P) -> io::Result<()>
where
    P: AsRef<Path>,
{
    if !path.as_ref().exists() {
        return Ok(());
    }

    println!("cargo:rerun-if-changed={}", path.as_ref().display());

    if !path.as_ref().is_dir() {
        return Ok(());
    }

    for path in fs::read_dir(path)?.map(|res| res.map(|e| e.path())) {
        rerun_if_changed(path?)?;
    }

    Ok(())
}
