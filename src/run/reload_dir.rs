use std::{
    fs::{read_dir, read_to_string},
    path::PathBuf,
};

use crate::module;
pub use crate::AbanModule;

pub fn reload_directory(
    reload_dir_error: &mut String,
    path: &PathBuf,
    modules: &mut Vec<AbanModule>,
) {
    reload_dir_error.clear();
    let _ = match read_to_string(path.clone()) {
        Ok(res) => res,
        Err(err) => {
            reload_dir_error.insert_str(0, format!("Open Failed: {}", err).as_str());
            return;
        }
    };

    let mut path = path.clone();
    path.pop();
    path.push("src-c");

    let read_dir = match read_dir(path.clone()) {
        Ok(read_dir) => read_dir,
        Err(err) => {
            reload_dir_error.insert_str(0, format!("Open 'src-c' Failed: {}", err).as_str());
            return;
        }
    };

    let mut aban_modules = Vec::<AbanModule>::new();

    for item in read_dir {
        aban_modules.push(module::load(&item, path.clone()));
    }

    modules.clear();
    modules.append(&mut aban_modules);
}
