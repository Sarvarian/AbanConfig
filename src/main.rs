use std::path::PathBuf;

use fltk::{
    app::App,
    dialog::{FileDialog, FileDialogType},
    enums::Shortcut,
    frame::Frame,
    menu::{MenuBar, MenuFlag},
    prelude::{GroupExt, MenuExt, WidgetExt},
    window::Window,
};

#[derive(Debug, Clone, Copy)]
pub enum Message {
    SelectDirectory,
}

struct AppState {
    dir: PathBuf,
    label_dir: Frame,
}

fn main() {
    // Build app.
    let app = App::default();

    // Build window.
    let mut window = Window::default()
        .with_size(400, 400)
        .with_label("Aban Config");

    // Message channel.
    let (sender, receiver) = fltk::app::channel::<Message>();

    // Build UI.
    MenuBar::default()
        .with_size(400, 20)
        .with_label("My Menu")
        .add_emit(
            "Open",
            Shortcut::None,
            MenuFlag::Normal,
            sender,
            Message::SelectDirectory,
        );

    let label_dir = Frame::default()
        .with_pos(0, 20)
        .with_size(400, 20)
        .with_label("");

    // Finish window
    window.end();
    window.show();

    // Build app state
    let mut state = AppState {
        dir: PathBuf::new(),
        label_dir,
    };

    // Run the app.
    while app.wait() {
        if let Some(msg) = receiver.recv() {
            match msg {
                Message::SelectDirectory => select_directory(&mut state),
            }
        }
    }
}

fn select_directory(state: &mut AppState) {
    let mut fd = FileDialog::new(FileDialogType::BrowseDir);
    fd.show();
    state.dir = fd.filename();
    println!("'{}'", state.dir.to_str().unwrap());
    state.label_dir.set_label(state.dir.to_str().unwrap());
}
