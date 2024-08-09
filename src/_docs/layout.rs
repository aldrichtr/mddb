//! # Vault Directory structure
//!
//! there are two basic types of layouts supported by `mddb`:
//!
//! - dendron: Single directory of files, where hierarchy is used to create the file name
//! - obsidian: Hierarchy is defined from directories within the vault
//!
//! ## Dendron type
//! [dendron](https://www.dendron.so) prescribes that a vault is a single folder of markdown files, with the file
//! names being constructed in a hierarchical manner where the levels are separated by a `.`.  For example:
//! `lang.python.data.boolean.md`.
//! [explanation](https://blog.dendron.so/notes/3dd58f62-fee5-4f93-b9f1-b0f0f59a9b64/)
