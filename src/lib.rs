/*!
 *
 */

use std::path::PathBuf;

use vault::Vault;

mod vault;
mod parser;
mod id;
mod error;

struct VaultData;


mod test {
    use super::*;
    #[cfg(test)]

    #[test]
    fn connect_to_vault() {
        let v = Vault::new("~/dendron/vaults/journal/notes", Some("*.md"), Some("journal"), None);
        v.parse();
    }
}
