use std::{
    fs::{create_dir_all, read_to_string},
    path::PathBuf,
};

use crate::{appinput::GenCMakeOptions, constants::*};

pub fn gen_cmake(_options: GenCMakeOptions) {
    // Create cmake directory.
    let path = PathBuf::from(DIR_CMAKE);
    create_dir_all(path).expect(format!("Failed to create '{}' directory.", DIR_CMAKE).as_str());

    // Read cmake template file.
    let path = PathBuf::from(format!("{}/{}", DIR_TEMPLATES, FILE_CMAKE_TEMPLATE));
    let template =
        read_to_string(path).expect(format!("Failed to open '{}'.", FILE_CMAKE_TEMPLATE).as_str());
}
