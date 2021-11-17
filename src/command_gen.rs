use std::{
    fs::{create_dir_all, read_dir, read_to_string, DirEntry},
    path::PathBuf,
};

use serde::Deserialize;

use crate::{appinput::GenOptions, constants::*};

pub fn gen(options: GenOptions) {
    let start_path = options.start_path.unwrap_or("./".into());

    // Gather C modules information.
    let aban = Aban::new(start_path.clone());

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

    let template_os = {
        let mut path = start_path.clone();
        path.push(DIR_TEMPLATES);
        path.push(FILE_TEMPLATE_OS);
        read_to_string(path.clone()).expect(format!("Failed to read '{:?}'", path).as_str())
    };

    // add_

    // Create cmake directory.
    create_dir_all(DIR_CMAKE)
        .expect(format!("Failed to create '{}' directory.", DIR_CMAKE).as_str());

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
    fn new(start_path: PathBuf) -> Self {
        let config = {
            let mut config_path = start_path.clone();
            config_path.push(FILE_CONFIG_ABAN);
            toml::from_str(
                &read_to_string(config_path.clone())
                    .expect(format!("Failed to read '{:?}'", config_path).as_str()),
            )
            .expect(format!("Failed to deserialize '{:?}' as an AbanConfig", config_path).as_str())
        };

        let modules = {
            let mut src_c_path = start_path.clone();
            src_c_path.push(DIR_SRC_C);
            read_dir(src_c_path.clone())
                .expect(format!("Failed to read '{:?}'.", src_c_path).as_str())
                .fold(Vec::new(), |mut v, r| {
                    if let Ok(dir_entry) = r {
                        if let Some(m) = AbanModule::new_c(dir_entry) {
                            v.push(m)
                        }
                    }
                    v
                })
        };

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

        let string_toml = match read_to_string((|| {
            let mut path = dir_entry.path();
            path.push(FILE_CONFIG_MODULE_ABAN);
            return path;
        })()) {
            Ok(res) => res,
            Err(err) => {
                eprintln!(
                    "'{:?}' Failed to read to string. Error: {}",
                    dir_entry.path(),
                    err
                );
                return None;
            }
        };

        let config = match toml::from_str(&string_toml) {
            Ok(res) => res,
            Err(err) => {
                eprintln!(
                    "'{:?}' Failed to deserialize to toml. Error: {}",
                    dir_entry.path(),
                    err
                );
                return None;
            }
        };

        let name = match dir_entry.file_name().to_str() {
            Some(res) => res.to_string(),
            None => {
                eprintln!(
                    "'{:?}' Failed to get file (aka directory) name.",
                    dir_entry.path(),
                );
                return None;
            }
        };
        if name.len() > 250 {
            eprintln!("'{:?}' Module name is more then 250 character. Make it shorter or increase the limit.", dir_entry.path());
            return None;
        }

        Some(Self { name, config })
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

// ----- Handlebars -----
struct HandlebarsPartOfThisApp {
    add_modules_init: String,
    add_modules_exit: String,
}
