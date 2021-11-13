mod config;

use std::fs::read_to_string;

use crate::app_state::AppState;

pub fn reload_directory(state: &mut AppState) {
    state.reload_dir_error.clear();
    let _ = match read_to_string(state.path.clone()) {
        Ok(res) => res,
        Err(err) => {
            state.reload_dir_error = format!("Open Failed: {}", err);
            return;
        }
    };

    let mut path = state.path.clone();
    path.pop();
    path.push("src-c");
}
