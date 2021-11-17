pub const TEMPLATE_C_MAKE_LIST_TXT: &str = r#"
cmake_minimum_required(VERSION 3.10)

project({{project_name}} VERSION 0.1.0)

set( CMAKE_RUNTIME_OUTPUT_DIRECTORY "../bin" )
set( CMAKE_LIBRARY_OUTPUT_DIRECTORY "../bin" )
set( CMAKE_ARCHIVE_OUTPUT_DIRECTORY "../bin" )

option(ABAN_WIN32_SUBSYSTEM_WINDOWS OFF)

if(ABAN_WIN32_SUBSYSTEM_WINDOWS)
    add_executable({{project_name}} WIN32 "")
else()
add_executable({{project_name}} "")
endif()

{{add_main}}

{{add_modules}}

"#;
