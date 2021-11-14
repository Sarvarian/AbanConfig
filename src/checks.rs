use fltk::browser::CheckBrowser;

pub struct Checks {
    browser: CheckBrowser,
    modules: i32,
    cmake: i32,
}

impl Checks {
    pub fn new(mut browser: CheckBrowser) -> Self {
        let modules = browser.add("Modules", true);
        let cmake = browser.add("CMake", true);

        Self {
            browser,
            modules,
            cmake,
        }
    }

    pub fn modules(&self) -> bool {
        self.browser.checked(self.modules)
    }

    pub fn cmake(&self) -> bool {
        self.browser.checked(self.cmake)
    }
}
