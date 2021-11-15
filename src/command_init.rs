use std::{
    env::current_dir,
    fs::{create_dir_all, write},
    path::PathBuf,
};

use crate::{appinput::InitOptions, constants::*};

pub fn init(_options: InitOptions) {
    init_dir("./".into());
}

pub fn init_dir(path: PathBuf) {
    // Get last directory name.
    let name = {
        let path = path.clone();
        match path.file_name() {
            Some(name) => name.to_owned(),
            None => current_dir()
                .expect("Failed to get current directory.")
                .file_name()
                .expect("Failed to get file_name from current_dir.")
                .to_owned(),
        }
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
        format!("name = \"{}\"", name.to_str().unwrap()),
    );

    // Create cmake templates.
    // TODO -- Create cmake templates.
}

fn init_create_file(path: &PathBuf, name: &str, contents: String) {
    let mut path = path.clone();
    path.push(name);
    write(path, contents).expect(format!("Failed to create '{}'", name).as_str());
}

fn init_create_dir(path: &PathBuf, name: &str) {
    let mut path = path.clone();
    path.push(name);
    create_dir_all(path).expect(format!("Failed to create '{}'", name).as_str());
}

// g
// g
// G
// G
