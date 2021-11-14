use app_state::build_app_state;
use fltk::{
    app::App,
    prelude::{GroupExt, WidgetExt},
    window::Window,
};
use message::Message;
use run::run;

pub use module::AbanModule;
pub use module_config::{AbanModuleConfig, AbanModuleConfigOS};

mod app_state;
mod checks;
mod message;
mod module;
mod module_config;
mod project;
mod run;

fn main() {
    // Build app.
    let app = App::default();

    // Build window.
    let mut window = Window::default()
        // .with_pos(50, 100)
        .with_size(800, 600)
        .with_label("Aban Config");

    // Message channel.
    let (sender, receiver) = fltk::app::channel::<Message>();

    // Build ui and app state.
    let state = build_app_state(sender);

    // Finish window
    window.end();
    window.show();

    // Run.
    run(&app, state, receiver);
}
