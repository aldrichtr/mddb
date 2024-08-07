use std::path::PathBuf;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum DataStoreError {
    #[error("Vault at {path:?} did not contain any parsable files")]
    EmptyVaultError { path : PathBuf },

    #[error("Error parsing file {fname:?}: {msg:?}")]
    VaultParseError { fname : String, msg : String },

    #[error("File '{fname:?}' already exists")]
    FileExistsError { fname : String },
}
