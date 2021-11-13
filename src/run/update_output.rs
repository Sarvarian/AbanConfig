use std::ops::Add;

use fltk::prelude::InputExt;

use crate::app_state::AppState;

pub fn update_output(state: &mut AppState) {
    let mut string = String::new();

    string = string.add("Path: ");
    string = string.add(state.path.to_str().unwrap());
    string = string.add(" ;");
    string = string.add("\n");
    if state.reload_dir_error.as_str() != "" {
        string = string.add(state.reload_dir_error.as_str());
        string = string.add("\n");
    }

    state.output.set_value(string.as_str());
}
