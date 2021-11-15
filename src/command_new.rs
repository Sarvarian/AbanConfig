use std::fs::create_dir_all;

use crate::{appinput::NewOptions, command_init::init_dir};

pub fn new(options: NewOptions) {
    // Create Project directory.
    create_dir_all(options.path.clone()).expect(
        format!(
            "Failed to create project directory at '{}'",
            options.path.to_str().unwrap()
        )
        .as_str(),
    );

    // Initialize directory.
    init_dir(options.path);
}
