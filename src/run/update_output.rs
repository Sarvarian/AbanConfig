use fltk::prelude::InputExt;

use crate::{app_state::AppState, AbanModule};

pub fn update_output(state: &mut AppState) {
    let mut string = String::new();

    string += "Path: ";
    string += state.path.to_str().unwrap();
    string += " ;";
    string += "\n";

    if !state.error_reload_dir.is_empty() {
        string += state.error_reload_dir.as_str();
        string += "\n";
    }

    if !state.error_gen_cmake.is_empty() {
        string += state.error_gen_cmake.as_str();
        string += "\n";
    }

    if !state.error_run_cmake.is_empty() {
        string += state.error_run_cmake.as_str();
        string += "\n";
    }

    if state.checks.modules() {
        string += add_module_information(&state.modules).as_str();
    }

    if state.checks.cmake() {
        if let Some(output) = &state.cmake_output {
            string += add_cmake_output(output).as_str();
        }
    }

    state.output.set_value(string.as_str());
}

fn add_cmake_output(output: &std::process::Output) -> String {
    let mut string = String::new();

    let add_exit_code = || {
        let mut string = String::new();
        match output.status.code() {
            Some(code) => string += format!("Exit Code: {}", code).as_str(),
            None => string += "Exit Code: None",
        }
        string += "\n";
        return string;
    };

    let add_stdout = || {
        let mut string = String::new();
        match std::str::from_utf8(&output.stdout) {
            Ok(res) => string += format!("\nSTDOUT:\n{}\n", res).as_str(),
            Err(err) => {
                string +=
                    format!("\nSTDOUT: Failed to interpret result as string: {}\n", err).as_str()
            }
        }
        return string;
    };

    let add_stderr = || {
        let mut string = String::new();
        match std::str::from_utf8(&output.stderr) {
            Ok(res) => string += format!("\nSTDERR:\n{}\n", res).as_str(),
            Err(err) => {
                string +=
                    format!("\nSTDERR: Failed to interpret result as string: {}\n", err).as_str()
            }
        }
        return string;
    };

    string += "\n\n----- CMake -----\n\n";
    string += format!("Success: {}\n", output.status.success()).as_str();
    string += add_exit_code().as_str();
    string += add_stdout().as_str();
    string += add_stderr().as_str();
    string += "\n-----------------------\n";

    string
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
