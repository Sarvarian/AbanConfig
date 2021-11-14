use crate::app_state::AppState;

pub fn run_cmake(state: &mut AppState) {
    state.error_run_cmake.clear();

    let mut path = state.path.clone();
    path.pop();
    path.push("cmake");

    let cmake_path = path.to_str().unwrap();

    let res = std::process::Command::new("cmake")
        .args(["-S", &cmake_path, "-B", &cmake_path])
        .output();

    match res {
        Ok(output) => state.cmake_output = Some(output),
        Err(err) => {
            state.error_run_cmake = format!("Running 'cmake' command failed: {}", err);
            return;
        }
    }
}
