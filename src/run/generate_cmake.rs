use std::fs::{create_dir_all, write};

use serde_derive::Serialize;
use tinytemplate::TinyTemplate;

use crate::app_state::AppState;

pub fn generate_cmake(state: &mut AppState) {
    if !state.is_valid {
        return;
    }

    let mut path = state.path.clone();
    path.pop();
    path.push("cmake");
    let cmake_path_clone = path.clone();
    if let Err(err) = create_dir_all(path.clone()) {
        state.error_gen_cmake = format!(
            "Failed to create '{}' directory. Error: {}",
            path.to_str().unwrap(),
            err
        );
        return;
    }
    path.push("CMakeLists.txt");

    let cmake_lists_txt = render(state.config.name.clone());
    let res = write(path.clone(), cmake_lists_txt);
    if let Err(err) = res {
        state.error_gen_cmake = format!(
            "Failed to write '{}' directory. Error: {}",
            path.to_str().unwrap(),
            err
        );
        return;
    }

    // let res = std::process::Command::new("cmake")
    //     .args(["-S", &cmake_path, "-B", &cmake_path])
    //     .output();

    // match res {
    //     Ok(output) => println!("Output: {:?}", output),
    //     Err(err) => println!("Error: {}", err),
    // }
}

fn render(project_name: String) -> String {
    let context = build_context(project_name);
    let mut tt = TinyTemplate::new();
    tt.add_template("cmake_list_txt", C_MAKE_LIST_TXT_TEMPLATE)
        .unwrap();
    let rendered = tt.render("cmake_list_txt", &context).unwrap();

    return rendered;
}

fn build_context(project_name: String) -> CMakeListTemplateContext {
    CMakeListTemplateContext {
        name: project_name.clone(),
        add_c_modules: "add_subdirectory(src-aban)".to_string(),
        add_main: "add_subdirectory(src-c)".to_string(),
        cmake_current_list_dir: "{CMAKE_CURRENT_LIST_DIR}".to_string(),
    }
}

#[derive(Serialize)]
struct CMakeListTemplateContext {
    name: String,
    add_main: String,
    add_c_modules: String,
    cmake_current_list_dir: String,
}

static C_MAKE_LIST_TXT_TEMPLATE: &'static str = r#"
cmake_minimum_required(VERSION 3.10)

project({name} VERSION 0.1.0)

set( CMAKE_RUNTIME_OUTPUT_DIRECTORY "${cmake_current_list_dir}/../bin" )
set( CMAKE_LIBRARY_OUTPUT_DIRECTORY "${cmake_current_list_dir}/../bin" )
set( CMAKE_ARCHIVE_OUTPUT_DIRECTORY "${cmake_current_list_dir}/../bin" )

option(ABAN_WIN32_SUBSYSTEM_WINDOWS OFF)

if(ABAN_WIN32_SUBSYSTEM_WINDOWS)
    add_executable({name} WIN32 "")
else()
    add_executable({name} "")
endif()

{add_main}
{add_c_modules}

"#;
