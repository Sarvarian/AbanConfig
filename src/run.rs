mod generate_cmake;
mod generate_src;
mod reload_dir;
mod run_cmake;
mod select_dir;
mod update_output;

use fltk::app::{App, Receiver};

use crate::{app_state::AppState, message::Message};

use self::{
    generate_cmake::generate_cmake, generate_src::generate_source, reload_dir::reload_directory,
    run_cmake::run_cmake, select_dir::select_directory, update_output::update_output,
};

pub fn run(app: &App, mut state: AppState, receiver: Receiver<Message>) {
    while app.wait() {
        if let Some(msg) = receiver.recv() {
            match msg {
                Message::SelectDirectory => {
                    select_directory(&mut state);
                    reload_directory(&mut state);
                }
                Message::ReloadDirectory => {
                    reload_directory(&mut state);
                }
                Message::GenerateSource => generate_source(&state),
                Message::GenerateCMake => generate_cmake(&mut state),
                Message::Check => (),
                Message::CMake => run_cmake(&mut state),
            } // match
            update_output(&mut state);
        } // if
    } // while
} // run
