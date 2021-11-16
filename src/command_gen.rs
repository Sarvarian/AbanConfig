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

    // Generate C OS Code.
    // For each module, if it has init and/or exit on true, add it to the init and/or exit list.
    // So we have two list. init list and exit list.
    // Check each module for init and exit and add them to the list if it was true.
    // Then generate code base on init and exit list.
    let (init_list, exit_list) = aban.modules.iter().fold(
        (
            Vec::with_capacity(aban.modules.len()),
            Vec::with_capacity(aban.modules.len()),
        ),
        |(mut v_init, mut v_exit), m| {
            if let Some(os) = &m.config.os {
                if os.init {
                    v_init.push(m.name.clone());
                }
                if os.exit {
                    v_exit.push(m.name.clone());
                }
            }
            (v_init, v_exit)
        },
    );

    let lists = [init_list, exit_list];

    for i in lists {}

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
    name: String,
    config: AbanModuleConfig,
}

impl AbanModule {
    fn new_c(dir_entry: DirEntry) -> Option<Self> {
        let is_dir = dir_entry.file_type().map(|ft| ft.is_dir()).unwrap_or(false);
        if !is_dir {
            println!("'{:?}' is not an directory.", dir_entry.path());
            return None;
        }

        let string = match read_to_string(dir_entry.path()) {
            Ok(res) => res,
            Err(err) => {
                println!(
                    "'{:?}' Failed to read to string. Error: {}",
                    dir_entry.path(),
                    err
                );
                return None;
            }
        };

        let config = match toml::from_str(&string) {
            Ok(res) => res,
            Err(err) => {
                println!(
                    "'{:?}' Failed to deserialize to toml. Error: {}",
                    dir_entry.path(),
                    err
                );
                return None;
            }
        };

        Some(Self {
            name: todo!(),
            config,
        })
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
