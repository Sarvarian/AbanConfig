// Constants.

// Directories

pub const DIR_ABAN: &str = "aban";
pub const DIR_TEMPLATES: &str = "aban-templates";
pub const DIR_CMAKE: &str = "cmake";
pub const DIR_SRC_MAIN: &str = "src-aban";
pub const DIR_SRC_C: &str = "src-c";
pub const DIR_SRC_RUST: &str = "src-rs";

// Files

pub const FILE_CONFIG_ABAN: &str = "aban.toml";
pub const FILE_TEMPLATE_CMAKE: &str = "CMakeLists.txt";

// Templates

pub const C_MAKE_LIST_TXT_TEMPLATE: &str = r#"
cmake_minimum_required(VERSION 3.10)

project({{name}} VERSION 0.1.0)

set( CMAKE_RUNTIME_OUTPUT_DIRECTORY "${cmake_current_list_dir}/../bin" )
set( CMAKE_LIBRARY_OUTPUT_DIRECTORY "${cmake_current_list_dir}/../bin" )
set( CMAKE_ARCHIVE_OUTPUT_DIRECTORY "${cmake_current_list_dir}/../bin" )

option(ABAN_WIN32_SUBSYSTEM_WINDOWS OFF)

if(ABAN_WIN32_SUBSYSTEM_WINDOWS)
    add_executable({{name}} WIN32 "")
else()
add_executable({{name}} "")
endif()

{{add_main}}
"#;
