mod select_dir;

use fltk::app::{App, Receiver};

use crate::{app_state::AppState, message::Message};

use self::select_dir::select_directory;

pub fn run(app: &App, mut state: AppState, receiver: Receiver<Message>) {
    while app.wait() {
        if let Some(msg) = receiver.recv() {
            match msg {
                Message::SelectDirectory => select_directory(&mut state),
            }
        }
    }
}
