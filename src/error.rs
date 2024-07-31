
use thiserror::Error;

#[derive(Debug, Error)]
pub enum MdDbError {
    #[error("Error parsing file {file}: {msg}")]
    VaultParseError(file: &str, msg: &str),
}
