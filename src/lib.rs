//  #![deny(missing_docs)]

//! > **Datastore provider using a collection of Markdown files as it's
//! > backend**

mod database;
mod error;
mod id;
mod log;
mod parser;
mod vault;

use lalrpop_util::lalrpop_mod;
pub use crate::{database::Database, id::Id, vault::Vault};


lalrpop_mod!(pub grammar); // synthesized by LALRPOP

#[cfg(test)]
mod tests {

    use super::Database;
    use k9::assert_equal;
    use log::debug;

    #[test_log::test]
    fn create_new_database() {
        let mut db = Database::new();
        db.execute(String::from("create table aldrichtr")).unwrap();
        db.execute(String::from(
            "insert into aldrichtr ('id', 'fname') values ('abc123', 'root'",
        ))
        .unwrap();
        debug!("database is {db:#?}");
        let fname = db
            .execute(String::from("select fname from aldrichtr"))
            .unwrap();
        assert_equal!(String::from("root"), fname);
    }
}
