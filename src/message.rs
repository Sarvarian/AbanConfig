#[derive(Debug, Clone, Copy)]
pub enum Message {
    SelectDirectory,
    ReloadDirectory,
    SourceGenerate,
    CMakeGenerate,
    Check,
}
