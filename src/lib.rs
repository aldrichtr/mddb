//  #![deny(missing_docs)]

//! Datastore provider using a collection of Markdown files as it's data.
//!
//! Recently there has been a bloom in systems that allow you to maintain a Knowledge Management System (KMS) using
//! markdown files organized on the filesystem in what they refer to as a "vault".  Obsidian is a recent and very
//! popular version, but there are many others such as logsec, roam-research, foam, and my personal favorite
//! [dendron](https://github.com/dendronhq/dendron).
//! Dendron introduced a hierarchical method of organizing the files by using a flat folder where the names of the
//! files represented their place in the "hierarchy".  For an in-depth article about the layout, see [this
//! post](https://blog.dendron.so/notes/3dd58f62-fee5-4f93-b9f1-b0f0f59a9b64/) by the creator Kevin Lin.
//!
//! There is a lot of (meta)data stored in those files, and it could be useful to present that data in a structured,
//! searchable way.  This library takes a collection of files (a vault) and presents them as tables in a database
//! that can be queried and manipulated using a "SQL-like" language.  It is SQL-like because the database only
//! recognizes a small subset of the SQL-Language.
//!
//! The data available varies depending on what is in the vault, and there is some configuration you can give the
//! parser to limit or transform that data as it gets loaded into the table.  But some things that make for good
//! datapoints are the filename/path of the file, the created and modified dates, the title, the headings, and any
//! data that is stored in the frontmatter.
//! Take this markdown file for example:
//! ```markdown
//! ---
//! id: 7csf5wyep1i96o3x35oalg5
//! title:
//! ```
mod database;
mod error;
mod log;
mod vault;

use lalrpop_util::lalrpop_mod;
pub use crate::{database::Database, vault::Vault};


lalrpop_mod!(grammar, "/database/grammar.rs"); // synthesized by LALRPOP

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
