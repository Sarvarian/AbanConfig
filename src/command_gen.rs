use std::{
    fs::{create_dir_all, read_dir, read_to_string, DirEntry},
    path::PathBuf,
};

use serde::{Deserialize, Serialize};

use crate::{appinput::GenOptions, constants::*};

pub fn gen(options: GenOptions) {
    let start_path = options.start_path.unwrap_or("./".into());

    // Gather C modules information.
    let aban = Aban::new(start_path.clone());

    // Load templates.
    let templates_information = load_templates(&start_path);

    // Register templates in an handlebars registry.
    let mut handlebars_registry = handlebars::Handlebars::new();
    for template_information in templates_information {
        handlebars_registry
            .register_template_string(
                template_information.name,
                template_information.template_string,
            )
            .expect(
                format!(
                    "Failed to register '{}' template in an Handlebars registry.",
                    template_information.name
                )
                .as_str(),
            );
    }

    // Store list of modules that requires init and/or exit.
    let (list_init, list_exit) = aban.modules.iter().fold(
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

    let string_module_init = {
        let mut string_module_init = String::new();
        for module_name in list_init {
            let data = ModuleTemplateData { module_name };
            string_module_init += &handlebars_registry
                .render("os_init", &data)
                .expect("Failed to render os init template with Handlebars.");
        }
        string_module_init
    };

    println!("{}", string_module_init);

    // Create cmake directory.
    create_dir_all(DIR_CMAKE)
        .expect(format!("Failed to create '{}' directory.", DIR_CMAKE).as_str());
}

fn load_templates(start_path: &PathBuf) -> Vec<AbanTemplateInformation> {
    let os = load_template(&start_path, FILE_TEMPLATE_OS);
    let os_init = load_template(&start_path, FILE_TEMPLATE_OS_ADD_MODULE_INIT);
    let os_exit = load_template(&start_path, FILE_TEMPLATE_OS_ADD_MODULE_EXIT);
    let cmake = load_template(&start_path, FILE_TEMPLATE_CMAKE);

    vec![os, os_init, os_exit, cmake]
}

fn load_template<'a>(start_path: &PathBuf, file_name: &'a str) -> AbanTemplateInformation<'a> {
    let mut path = start_path.clone();
    path.push(DIR_TEMPLATES);
    path.push(file_name);
    let template_string =
        read_to_string(path.clone()).expect(format!("Failed to read '{:?}'", path).as_str());

    AbanTemplateInformation {
        name: file_name,
        template_string,
    }
}

// ----- Aban Template Information -----
struct AbanTemplateInformation<'a> {
    name: &'a str,
    template_string: String,
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

        let string_toml = {
            let mut path = dir_entry.path();
            path.push(FILE_CONFIG_MODULE_ABAN);
            match read_to_string(path.clone()) {
                Ok(res) => res,
                Err(err) => {
                    eprintln!("'{:?}' Failed to read to string. Error: {}", path, err);
                    return None;
                }
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

#[derive(Serialize)]
struct ModuleTemplateData {
    module_name: String,
}

// g
// g
// G
// G
