use std::path::PathBuf;

use fltk::{
    app::Sender,
    browser::CheckBrowser,
    button::Button,
    enums::CallbackTrigger,
    output::MultilineOutput,
    prelude::{InputExt, WidgetExt},
};

use crate::{checks::Checks, message::Message, project::AbanProjectConfig, AbanModule};

pub struct AppState {
    pub is_valid: bool,
    pub path: PathBuf,
    pub error_reload_dir: String,
    pub error_gen_cmake: String,
    pub output: MultilineOutput,
    pub config: AbanProjectConfig,
    pub modules: Vec<AbanModule>,
    pub checks: Checks,
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
        .with_label("Source Generate");
    button.emit(sender, Message::SourceGenerate);
    button_pos_y += BUTTON_HEIGHT;

    let mut button = Button::default()
        .with_pos(button_pos_x, button_pos_y)
        .with_size(BUTTON_WIDTH, BUTTON_HEIGHT)
        .with_label("CMake Generate");
    button.emit(sender, Message::CMakeGenerate);
    button_pos_y += BUTTON_HEIGHT;

    let mut check_browser = CheckBrowser::default()
        .with_pos(0, button_pos_y)
        .with_size(BUTTON_WIDTH, BUTTON_HEIGHT);
    check_browser.emit(sender, Message::Check);
    check_browser.set_trigger(CallbackTrigger::Changed);
    let checks = Checks::new(check_browser);

    let mut output = MultilineOutput::default()
        .with_pos(200, 0)
        .with_size(600, 600);
    output.set_value("Hello!");
    output.set_wrap(true);

    AppState {
        is_valid: false,
        path: PathBuf::new(),
        error_reload_dir: String::new(),
        error_gen_cmake: String::new(),
        output,
        config: AbanProjectConfig::default(),
        modules: Vec::new(),
        checks,
    }
}
