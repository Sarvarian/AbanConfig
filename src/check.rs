use fltk::{app::Sender, browser::CheckBrowser, prelude::WidgetExt};

use crate::message::Message;

pub struct Checks {
    browser: CheckBrowser,
    modules: i32,
}

impl Checks {
    pub fn new(pos_y: i32, width: i32, height: i32, sender: Sender<Message>) -> Self {
        let mut browser = CheckBrowser::default();
        browser.set_pos(0, pos_y);
        browser.set_size(width, height);
        browser.emit(sender, Message::Check);

        let modules = browser.add("Modules", true);

        Self { browser, modules }
    }

    pub fn modules(&self) -> bool {
        self.browser.checked(self.modules)
    }
}
