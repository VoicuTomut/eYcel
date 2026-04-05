use thiserror::Error;

#[derive(Error, Debug)]
pub enum EYcelError {
    #[error("Encryption error: {0}")]
    Encryption(String),

    #[error("Decryption error: {0}")]
    Decryption(String),

    #[error("Rules validation error: {0}")]
    RulesValidation(String),

    #[error("Formula error: {0}")]
    Formula(String),

    #[error("IO error: {0}")]
    Io(String),

    #[error("Excel parse error: {0}")]
    ExcelParse(String),
}
