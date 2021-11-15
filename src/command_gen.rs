use std::{
    fs::{create_dir_all, read_dir, read_to_string},
    path::PathBuf,
};

use serde::Deserialize;

use crate::{appinput::GenOptions, constants::*};

pub fn gen(_options: GenOptions) {
    // Create cmake directory.
    let path = PathBuf::from(DIR_CMAKE);
    create_dir_all(path).expect(format!("Failed to create '{}' directory.", DIR_CMAKE).as_str());

    // Read aban.toml file.
    let aban_toml = read_to_string(FILE_CONFIG_ABAN)
        .expect(format!("Failed to read '{}'", FILE_CONFIG_ABAN).as_str());

    // Create config structure from aban_toml.
    let aban_config: AbanConfig = toml::from_str(&aban_toml).expect(
        format!(
            "Failed to deserialize '{}' as an AbanConfig",
            FILE_CONFIG_ABAN
        )
        .as_str(),
    );

    // Gather C modules information.
    let aban_modules_information = {
        // Search src-c directory for subdirectories.
        let rd = read_dir(DIR_SRC_C).expect(format!("Failed to read '{}'.", DIR_SRC_C).as_str());

        // Iterate trough src-c directories.
        for res in rd {
            if let Ok(entry) = res {
                entry.
            }
        }

        return ();
    };

    let aban = Aban {
        config: aban_config,
        modules: todo!(),
    };

    // Read cmake template file.
    let path = PathBuf::from(format!("{}/{}", DIR_TEMPLATES, FILE_TEMPLATE_CMAKE));
    let template =
        read_to_string(path).expect(format!("Failed to open '{}'.", FILE_TEMPLATE_CMAKE).as_str());
}

struct Aban {
    config: AbanConfig,
    modules: Vec<AbanModule>,
}

#[derive(Deserialize)]
struct AbanConfig {
    name: String,
}

struct AbanModule {}

#[derive(Deserialize)]
struct AbanModuleConfig {
    os: Option<AbanOSConfig>,
}

#[derive(Deserialize)]
struct AbanOSConfig {
    init: bool,
    exit: bool,
}
