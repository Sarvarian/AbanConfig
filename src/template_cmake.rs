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
