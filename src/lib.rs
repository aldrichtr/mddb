//  #![deny(missing_docs)]

//! > **Datastore provider using a collection of Markdown files as it's backend**
//!

mod vault;
mod parser;
mod id;
mod error;

pub use crate::{
    vault::Vault,
    id::Id
};


mod test {
    #[cfg(test)]
    use super::vault::Vault;

    #[test]
    fn can_connect_to_vault() {
        let v = Vault::connect(
            "~/dendron/vaults/journal/notes",
            Some("*.md"),
            Some("journal"), None);
        assert_eq!("~/dendron/vaults/journal/notes", v.base.display().to_string().as_str());
    }
}
