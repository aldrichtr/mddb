//  #![deny(missing_docs)]

//! > **Datastore provider using a collection of Markdown files as it's
//! > backend**

mod error;
mod id;
mod parser;
mod vault;
mod log;
mod database;

pub use crate::{id::Id, vault::Vault, database::Database};


#[cfg(test)]
mod tests {

    use super::Database;
    use log::debug;
    use k9::assert_equal;

    #[test_log::test]
    fn create_new_database() {
        let mut db = Database::new();
        db.execute(String::from("create table aldrichtr")).unwrap();
        db.execute(String::from("insert into aldrichtr (id, fname) values (abc123, root")).unwrap();
        debug!("database is {db:#?}");
        let fname = db.execute(String::from("select fname from aldrichtr")).unwrap();
        assert_equal!(String::from("root"), fname);
    }

}
