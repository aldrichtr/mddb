
use thiserror::Error;

#[derive(Error, Debug)]
pub enum DataStoreError {

    #[error("Error parsing file {fname:?}: {msg:?}")]
    VaultParseError {
        fname: String,
        msg: String
    },

    #[error("File '{fname:?}' already exists")]
    FileExistsError {
        fname: String
    },
}
