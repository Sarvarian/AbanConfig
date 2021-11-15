use std::{
    fs::{create_dir_all, read_to_string},
    path::PathBuf,
};

use crate::{appinput::GenOptions, constants::*};

pub fn gen(_options: GenOptions) {
    // Create cmake directory.
    let path = PathBuf::from(DIR_CMAKE);
    create_dir_all(path).expect(format!("Failed to create '{}' directory.", DIR_CMAKE).as_str());

    // Read cmake template file.
    let path = PathBuf::from(format!("{}/{}", DIR_TEMPLATES, FILE_TEMPLATE_CMAKE));
    let template =
        read_to_string(path).expect(format!("Failed to open '{}'.", FILE_TEMPLATE_CMAKE).as_str());

    // Read aban.toml file.
    let aban_toml = read_to_string(FILE_CONFIG_ABAN)
        .expect(format!("Failed to read '{}'", FILE_CONFIG_ABAN).as_str());

    // Create config structure from aban_toml.
}
