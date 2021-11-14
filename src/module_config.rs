use serde_derive::Deserialize;

#[derive(Deserialize)]
pub struct AbanModuleConfig {
    os: AbanModuleConfigOS,
}

impl Default for AbanModuleConfig {
    fn default() -> Self {
        Self {
            os: AbanModuleConfigOS::default(),
        }
    }
}

#[derive(Deserialize)]
pub struct AbanModuleConfigOS {
    init: bool,
    exit: bool,
}

impl Default for AbanModuleConfigOS {
    fn default() -> Self {
        Self {
            init: false,
            exit: false,
        }
    }
}
