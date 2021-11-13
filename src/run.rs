mod generate;
mod reload_dir;
mod select_dir;
mod update_output;

use fltk::app::{App, Receiver};

use crate::{app_state::AppState, message::Message};

use self::{
    generate::generate, reload_dir::reload_directory, select_dir::select_directory,
    update_output::update_output,
};

pub fn run(app: &App, mut state: AppState, receiver: Receiver<Message>) {
    while app.wait() {
        if let Some(msg) = receiver.recv() {
            match msg {
                Message::SelectDirectory => {
                    select_directory(&mut state);
                    reload_directory(&mut state)
                }
                Message::ReloadDirectory => reload_directory(&mut state),
                Message::Generate => generate(&mut state),
            }
            update_output(&mut state);
        }
    }
}
