use std::path::PathBuf;

use fltk::{app::Sender, button::Button, prelude::WidgetExt};

use crate::message::Message;

pub struct AppState {
    pub dir: PathBuf,
}

pub fn build_app_state(sender: Sender<Message>) -> AppState {
    let mut button = Button::default()
        .with_pos(0, 0)
        .with_size(200, 20)
        .with_label("Select Project Directory");
    button.emit(sender, Message::SelectDirectory);

    AppState {
        dir: PathBuf::new(),
    }
}
