#[derive(Debug, Clone, Copy)]
pub enum Message {
    SelectDirectory,
    ReloadDirectory,
    GenerateSource,
    GenerateCMake,
    Check,
    CMake,
}
