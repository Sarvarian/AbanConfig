use serde_derive::Deserialize;

#[derive(Deserialize)]
pub struct AbanProjectConfig {
    pub name: String,
}

impl Default for AbanProjectConfig {
    fn default() -> Self {
        Self {
            name: "Aban".to_string(),
        }
    }
}
