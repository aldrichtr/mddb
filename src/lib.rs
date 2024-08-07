//  #![deny(missing_docs)]

//! > **Datastore provider using a collection of Markdown files as it's
//! > backend**

mod error;
mod id;
mod parser;
mod vault;

pub use crate::{id::Id, vault::Vault};
