use std::fs::{read_dir, read_to_string};

pub use crate::AbanModule;
use crate::{app_state::AppState, module};

pub fn reload_directory(state: &mut AppState) {
    let error_reload_dir = &mut state.error_reload_dir;
    let path = &state.path;
    let modules = &mut state.modules;

    error_reload_dir.clear();

    let res = match read_to_string(path.clone()) {
        Ok(res) => res,
        Err(err) => {
            error_reload_dir.insert_str(0, format!("Open Failed: {}", err).as_str());
            return;
        }
    };
    state.config = match toml::from_str(&res) {
        Ok(res) => res,
        Err(err) => {
            error_reload_dir.insert_str(
                0,
                format!("Reading project toml file failed: {}", err).as_str(),
            );
            return;
        }
    };

    let mut path = path.clone();
    path.pop();
    path.push("src-c");

    let read_dir = match read_dir(path.clone()) {
        Ok(read_dir) => read_dir,
        Err(err) => {
            error_reload_dir.insert_str(0, format!("Open 'src-c' Failed: {}", err).as_str());
            return;
        }
    };

    let mut aban_modules = Vec::<AbanModule>::new();

    for item in read_dir {
        aban_modules.push(module::load(&item, path.clone()));
    }

    modules.clear();
    modules.append(&mut aban_modules);

    state.is_valid = true;
}
