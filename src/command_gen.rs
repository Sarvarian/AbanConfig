use std::{
    fs::{create_dir_all, read_dir, read_to_string, DirEntry},
    path::PathBuf,
};

use serde::Deserialize;

use crate::{appinput::GenOptions, constants::*};

pub fn gen(_options: GenOptions) {
    // Create cmake directory.
    let path = PathBuf::from(DIR_CMAKE);
    create_dir_all(path).expect(format!("Failed to create '{}' directory.", DIR_CMAKE).as_str());

    // Gather C modules information.

    let aban = Aban::new();

    // Read cmake template file.
    let path = PathBuf::from(format!("{}/{}", DIR_TEMPLATES, FILE_TEMPLATE_CMAKE));
    let template =
        read_to_string(path).expect(format!("Failed to open '{}'.", FILE_TEMPLATE_CMAKE).as_str());
}

// ----- Aban -----
struct Aban {
    config: AbanConfig,
    modules: Vec<AbanModule>,
}

impl Aban {
    fn new() -> Self {
        let config = toml::from_str(
            &read_to_string(FILE_CONFIG_ABAN)
                .expect(format!("Failed to read '{}'", FILE_CONFIG_ABAN).as_str()),
        )
        .expect(
            format!(
                "Failed to deserialize '{}' as an AbanConfig",
                FILE_CONFIG_ABAN
            )
            .as_str(),
        );

        let modules = read_dir(DIR_SRC_C)
            .expect(format!("Failed to read '{}'.", DIR_SRC_C).as_str())
            .fold(Vec::new(), |mut v, r| {
                if let Ok(dir_entry) = r {
                    if let Some(m) = AbanModule::new_c(dir_entry) {
                        v.push(m)
                    }
                }
                v
            });

        Self { config, modules }
    }
}

// ----- Aban Config -----
#[derive(Deserialize)]
struct AbanConfig {
    pub name: String,
}

// ----- Aban Module -----
struct AbanModule {
    config: AbanModuleConfig,
}

impl AbanModule {
    fn new_c(dir_entry: DirEntry) -> Option<Self> {
        let is_dir = dir_entry.file_type().map(|ft| ft.is_dir()).unwrap_or(false);
        if !is_dir {
            println!("'{:?}' is not an directory.", dir_entry.path());
            return None;
        }

        // config = ;

        // Some(Self { config })

        None
    }
}

// ----- Aban Module Config -----
#[derive(Deserialize)]
struct AbanModuleConfig {
    pub os: Option<OSConfig>,
}

// ----- Aban Module OS Config
#[derive(Deserialize)]
struct OSConfig {
    pub init: bool,
    pub exit: bool,
}
