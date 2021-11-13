#[derive(Debug, Clone, Copy)]
pub enum Message {
    SelectDirectory,
    ReloadDirectory,
    Generate,
}
