use fltk::{app::Sender, browser::CheckBrowser, prelude::WidgetExt};

use crate::message::Message;

pub struct Checks {
    browser: CheckBrowser,
    modules: i32,
}

impl Checks {
    pub fn new(mut browser: CheckBrowser) -> Self {
        let modules = browser.add("Modules", true);

        Self { browser, modules }
    }

    pub fn modules(&self) -> bool {
        self.browser.checked(self.modules)
    }
}
