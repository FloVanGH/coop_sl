use std::{env, fs, io, path::*};

fn main() -> io::Result<()> {
    let include_file = "co_widgets.slint";
    let lib_name = "co_widgets";

    let output_path = Path::new(&env::var_os("OUT_DIR").ok_or(io::Error::new(
        io::ErrorKind::Other,
        "Cannot read output path",
    ))?)
    .join("");

    // copies all libraries files to output directory
    copy(lib_name, output_path.clone())?;

    // copy the main include library file to output directory
    fs::copy(
        include_file,
        output_path.join(Path::new(include_file).file_name().unwrap()),
    )?;

    println!("cargo:rustc-env=CO_WIDGETS_LIB_NAME={}", lib_name);
    println!(
        "cargo:rustc-env=CO_WIDGETS_INCLUDE_PATH={}",
        output_path.join(lib_name).as_path().display()
    );
    println!(
        "cargo:rustc-env=CO_WIDGETS_INCLUDE_FILE={}",
        output_path.join(include_file).as_path().display()
    );

    Ok(())
}

fn copy<P, Q>(from: P, output: Q) -> io::Result<()>
where
    P: AsRef<Path>,
    Q: AsRef<Path>,
{
    if !from.as_ref().exists() {
        return Ok(());
    }

    println!("cargo:rerun-if-changed={}", from.as_ref().display());

    if !from.as_ref().is_dir() {
        fs::copy(
            from.as_ref(),
            output.as_ref().join(from.as_ref().file_name().unwrap()),
        )?;
        return Ok(());
    }

    // copy folder to output directory.
    let parent = output.as_ref().join(from.as_ref().file_name().unwrap());
    if parent.exists() {
        fs::remove_dir_all(parent.clone())?;
    }

    fs::create_dir(parent.clone())?;

    for path in fs::read_dir(from)?.map(|res| res.map(|e| e.path())) {
        copy(path?, &parent)?;
    }

    Ok(())
}
