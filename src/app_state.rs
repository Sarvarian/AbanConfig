use std::path::PathBuf;

use fltk::{
    app::Sender,
    button::Button,
    output::MultilineOutput,
    prelude::{InputExt, WidgetExt},
};

use crate::{message::Message, AbanModule};

pub struct AppState {
    pub path: PathBuf,
    pub reload_dir_error: String,
    pub output: MultilineOutput,
    pub modules: Vec<AbanModule>,
}

pub fn build_app_state(sender: Sender<Message>) -> AppState {
    const BUTTON_HEIGHT: i32 = 50;
    const BUTTON_WIDTH: i32 = 200;
    let button_pos_x = 0;
    let mut button_pos_y = 0;

    let mut button = Button::default()
        .with_pos(button_pos_x, button_pos_y)
        .with_size(BUTTON_WIDTH, BUTTON_HEIGHT)
        .with_label("Select Project Directory");
    button.emit(sender, Message::SelectDirectory);
    button_pos_y += BUTTON_HEIGHT;

    let mut button = Button::default()
        .with_pos(button_pos_x, button_pos_y)
        .with_size(BUTTON_WIDTH, BUTTON_HEIGHT)
        .with_label("Reload Project Directory");
    button.emit(sender, Message::ReloadDirectory);
    button_pos_y += BUTTON_HEIGHT;

    let mut button = Button::default()
        .with_pos(button_pos_x, button_pos_y)
        .with_size(BUTTON_WIDTH, BUTTON_HEIGHT)
        .with_label("Generate");
    button.emit(sender, Message::Generate);
    // button_pos_y += BUTTON_HEIGHT;

    let mut output = MultilineOutput::default()
        .with_pos(200, 0)
        .with_size(600, 600);
    output.set_value("Hello!");
    output.set_wrap(true);

    AppState {
        path: PathBuf::new(),
        reload_dir_error: String::new(),
        output,
        modules: Vec::new(),
    }
}
