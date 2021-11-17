use std::{
    env::current_dir,
    fs::{create_dir_all, write},
    path::PathBuf,
};

use crate::{appinput::InitOptions, constants::*, template_cmake::*, template_os::*};

pub fn init(_options: InitOptions) {
    init_dir("./".into());
}

pub fn init_dir(path: PathBuf) {
    // Get last directory name.
    let name_dir_project = || {
        path.clone()
            .file_name()
            .map(|o| o.to_owned())
            .unwrap_or_else(|| {
                current_dir()
                    .expect("Failed to get current directory.")
                    .file_name()
                    .expect("Failed to get file_name from current_dir.")
                    .to_owned()
            })
            .to_str()
            .unwrap()
            .to_string()
    };

    // Create aban directory.
    init_create_dir(&path, DIR_ABAN);

    // Create cmake directory.
    init_create_dir(&path, DIR_CMAKE);

    // Create templates directory.
    init_create_dir(&path, DIR_TEMPLATES);

    // Create src-aban directory.
    init_create_dir(&path, DIR_SRC_MAIN);

    // Create src-c directory.
    init_create_dir(&path, DIR_SRC_C);

    // Create src-rs directory.
    init_create_dir(&path, DIR_SRC_RUST);

    // Create aban.toml file.
    init_create_file(
        &path,
        FILE_CONFIG_ABAN,
        format!("name = \"{}\"", name_dir_project()),
    );

    // Create cmake templates.
    init_create_file(
        &path,
        format!("{}/{}", DIR_TEMPLATES, FILE_TEMPLATE_CMAKE).as_str(),
        TEMPLATE_C_MAKE_LIST_TXT.into(),
    );

    // Create os templates.
    init_create_file(
        &path,
        format!("{}/{}", DIR_TEMPLATES, FILE_TEMPLATE_OS).as_str(),
        TEMPLATE_OS_PROCESS_MODULES_H.into(),
    );
}

fn init_create_file(path: &PathBuf, name: &str, contents: String) {
    let mut path = path.clone();
    path.push(name);
    if path.is_file() {
        println!(
            "Creating file '{}' skipped because such file already exists.",
            name
        );
    } else {
        write(path, contents).expect(format!("Failed to create '{}'", name).as_str());
        println!("File '{}' created.", name);
    }
}

fn init_create_dir(path: &PathBuf, name: &str) {
    let mut path = path.clone();
    path.push(name);
    create_dir_all(path).expect(format!("Failed to create '{}'", name).as_str());
    println!("Directory '{}' created.", name);
}

// g
// g
// G
// G
