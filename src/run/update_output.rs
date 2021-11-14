use fltk::prelude::InputExt;

use crate::{app_state::AppState, AbanModule};

pub fn update_output(state: &mut AppState) {
    let mut string = String::new();

    string += "Path: ";
    string += state.path.to_str().unwrap();
    string += " ;";
    string += "\n";
    if state.reload_dir_error.as_str() != "" {
        string += state.reload_dir_error.as_str();
        string += "\n";
    }

    // string = string.add("\n\n----- Aban Modules -----\n\n");
    // for m in state.modules.iter() {
    //     string = string.add(&add_module_information(m));
    // }
    // string = string.add("\n------------------------------\n");

    string += add_module_information(&state.modules).as_str();

    state.output.set_value(string.as_str());
}

fn add_module_information(modules: &Vec<AbanModule>) -> String {
    let mut string = String::new();

    let mut valid = Vec::new();
    let mut error = Vec::new();
    let mut invalid = Vec::new();

    for m in modules.iter() {
        if m.is_valid {
            if m.error.is_empty() {
                valid.push(m);
            } else {
                error.push(m);
            }
        } else {
            invalid.push(m);
        }
    }

    let add_valid = || {
        let mut string = String::new();
        for m in valid {
            string += format!("Module: {} (Valid)\n", m.path.to_str().unwrap()).as_str();
        }
        string
    };

    let add_error = || {
        let mut string = String::new();
        for m in error {
            string += "\n";
            string += format!(
                "Module: {} (Valid)\nError: {}",
                m.path.to_str().unwrap(),
                m.error
            )
            .as_str();
            string += "\n";
        }
        string
    };

    let add_invalid = || {
        let mut string = String::new();
        for m in invalid {
            string += "\n";
            string += format!(
                "Module: {} ( Not Valid)\nError: {}",
                m.path.to_str().unwrap(),
                m.error
            )
            .as_str();
            string += "\n";
        }
        string
    };

    string += "\n\n----- Aban Modules -----\n\n";
    string += add_valid().as_str();
    string += add_error().as_str();
    string += add_invalid().as_str();
    string += "\n------------------------------\n";

    return string;
}

// fn add_module_information(module: &AbanModule) -> String {
//     let mut string = String::new();
//     let validity = match module.is_valid {
//         true => "(Valid)",
//         false => "(Not Valid)",
//     };

//     string =
//         string.add(format!("Module: {} {}\n", module.path.to_str().unwrap(), validity).as_str());

//     if !module.error.is_empty() {
//         string = string.add(format!("Error: {}\n\n", module.error).as_str());
//     }

//     return string;
// }
