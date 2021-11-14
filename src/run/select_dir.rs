use fltk::dialog::{FileDialog, FileDialogType};

use crate::app_state::AppState;

pub fn select_directory(state: &mut AppState) {
    let mut fd = FileDialog::new(FileDialogType::BrowseFile);
    fd.set_filter("aban.toml");
    fd.show();
    if fd.filename().to_str().unwrap() == "" {
        return;
    }
    state.path = fd.filename();
}
