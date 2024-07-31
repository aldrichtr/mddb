use std::collections::BTreeMap;
// region: imports
//- stdlib
use std::{default::Default, fs, str::FromStr};

//- crates
use glob::{glob_with, MatchOptions, Paths, PatternError};
use regex::Regex;
//- local
use crate::id::Id;
use crate::error::MdDbError::VaultParseError;
use crate::parser;
// endregion imports

use std::path::PathBuf;

#[derive(Debug)]
pub struct Vault {
    pub name: String,
    pub path: PathBuf,
    pub pattern: String,
    pub options: MatchOptions,
    // index of ids to files for fast lookups of id
    // also used to lookup if an id is already in use
    index: BTreeMap<String , PathBuf>,
}

impl Default for Vault {
    fn default() -> Self {
        Self {
            name: String::from(""),
            path: PathBuf::new(),
            pattern: String::from("*.md"),
            options: MatchOptions::new(),
            index: BTreeMap::new()
        }
    }
}

impl Vault {
    pub fn new(
        path: &str,
        pattern: Option<&str>,
        name: Option<&str>,
        options: Option<MatchOptions>,
    ) -> Self {
        let _pattern = pattern.unwrap_or("*.md");
        let _name = name.unwrap_or("");
        let _options = options.unwrap_or(MatchOptions::new());
        // TODO: we should expand the path first
        let _path = PathBuf::from_str(path).expect("Could not create vault from path");

        Self {
            name: _name.to_string(),
            path: _path,
            pattern: _pattern.to_string(),
            options: _options,
            index: BTreeMap::new()
        }
    }

    pub fn parse(&self) -> Result<(), VaultParseError> {
        let parser = parser::Parser::new();
    }

    pub fn get_files(&self) -> Result<Paths, PatternError> {
        let path = self.path.join(self.pattern.clone()).display().to_string();
        glob_with(path.as_str(), self.options)
    }

    pub fn has_root(&self) -> bool {
        let root_file = "root.md";
        self.path.join(root_file).try_exists().unwrap()
    }

    pub fn get_root_id(&self) -> String {
        let root_file = self.path.join("root.md");
        let id = match root_file.try_exists() {
            Ok(true) => {
                let content = fs::read_to_string(root_file)?;
                let re = Regex::new(r"id = (\w+)$").unwrap();
                let matches = re.captures(&content).unwrap();
                let id = matches.get(1).map_or("", |m| m.as_str());

                if id.len() > 0 {
                    &id.to_string()
                } else {
                    Id::default().to_string()
                }
            }
            Ok(false) => Id::default().to_string(),
            Err(e) => Id::default().to_string(),
        };
        id.clone()
    }
}
// region: Tests

#[cfg(test)]
mod test {
    use super::*;
    use std::env;

    fn get_data_dir() -> PathBuf {
        let workspace = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
        workspace.join("test/data")
    }

    #[test]
    fn new_vault_with_no_params() {
        let out_dir = env!("TEMP");
        let expected = PathBuf::from_str(out_dir).expect("Could not create path");
        let v = Vault::new(&out_dir, Some("*.md"), Some("default"), None);
        assert_eq!(expected, v.path);
    }

    #[test]
    fn new_vault_get_files() {
        let binding = get_data_dir();
        let data_dir = binding.display();
        let v = Vault::new(data_dir.to_string().as_str(), None, Some("test_data"), None);
        let i = v.get_files().unwrap();
        assert_eq!(3, i.count(), "There should be {} files", 3);
        assert_eq!("test_data", v.name);
    }
}
// endregion Tests
