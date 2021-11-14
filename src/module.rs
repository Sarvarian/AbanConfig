use std::{
    fs::{read_to_string, DirEntry},
    path::PathBuf,
};

use crate::AbanModuleConfig;

pub struct AbanModule {
    is_valid: bool,
    path: PathBuf,
    name: String,
    error: String,
    toml: String,
    config: AbanModuleConfig,
}

pub fn load(item: &Result<DirEntry, std::io::Error>, path: PathBuf) -> AbanModule {
    let mut aban_module = AbanModule {
        is_valid: false,
        path,
        name: String::new(),
        error: String::new(),
        toml: String::new(),
        config: AbanModuleConfig::default(),
    };

    let item = match item {
        Ok(item) => item,
        Err(err) => {
            aban_module.error = format!("Read one of 'src-c' DirEntry Failed: {}", err);
            return aban_module;
        }
    }; // item

    let item_name = item.file_name();
    aban_module.name = item_name.to_str().unwrap().to_string();
    aban_module.path.push(item_name.clone());

    let file_type = match item.file_type() {
        Ok(file_type) => file_type,
        Err(err) => {
            aban_module.error = format!(
                "Read FileType of entry '{}' in 'src-c' Failed: {}",
                item_name.to_str().unwrap(),
                err
            );
            return aban_module;
        }
    }; // file type

    if !file_type.is_dir() {
        aban_module.error = format!("'{}' is not a directory.", item_name.to_str().unwrap());
        return aban_module;
    }

    let mut path = aban_module.path.clone();
    path.push(item_name.clone());
    path.push("aban.mod.toml");

    let res = match read_to_string(path.clone()) {
        Ok(res) => res,
        Err(err) => {
            aban_module.error = format!(
                "Failed to read '{}' to string: {}",
                path.to_str().unwrap(),
                err
            );
            return aban_module;
        }
    };

    aban_module.toml = res.clone();

    let config: AbanModuleConfig = match toml::from_str(&res) {
        Ok(config) => config,
        Err(err) => {
            aban_module.error = format!(
                "Reading '{}' module toml file failed: {}",
                item_name.to_str().unwrap(),
                err
            );
            return aban_module;
        }
    };

    aban_module.config = config;
    aban_module.is_valid = true;

    return aban_module;
}
