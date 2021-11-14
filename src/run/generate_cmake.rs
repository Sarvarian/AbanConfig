use serde_derive::Serialize;
use tinytemplate::TinyTemplate;

use crate::app_state::AppState;

#[derive(Serialize)]
struct Test {}

pub fn generate_cmake(state: &AppState) {
    if !state.is_valid {
        return;
    }
    let context = CMakeListTemplateContext {
        name: state.project.name.clone(),
        add_c_modules: "add_subdirectory(src-aban)".to_string(),
        add_main: "add_subdirectory(src-c)".to_string(),
        cmake_current_list_dir: "{CMAKE_CURRENT_LIST_DIR}".to_string(),
    };
    let mut tt = TinyTemplate::new();
    tt.add_template("cmake_list_txt", C_MAKE_LIST_TXT_TEMPLATE)
        .unwrap();
    let rendered = tt.render("cmake_list_txt", &context).unwrap();
    print!("{}", rendered);
}

fn render(project_name: String) -> String {}

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
