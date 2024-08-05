// region: imports
//- stdlib
use std::{
    collections::BTreeMap, default::Default, fs, path::{Path, PathBuf}, str::FromStr
};

//- crates
use glob::{glob_with, MatchOptions, Paths, PatternError};
use regex::Regex;
use relative_path::{FromPathError, RelativePath, RelativePathBuf};
//- local
use crate::id::Id;

use crate::error::DataStoreError;

// endregion imports

#[derive(Debug)]
pub struct Vault {
    pub name: String,
    pub base: PathBuf,
    pub pattern: String,
    pub options: MatchOptions,
    // index of ids to files for fast lookups of id
    // also used to lookup if an id is already in use
    index: BTreeMap<String, PathBuf>,
}

impl Default for Vault {
    fn default() -> Self {
        Self {
            name: String::from(""),
            base: PathBuf::new(),
            pattern: String::from("*.md"),
            options: MatchOptions::new(),
            index: BTreeMap::new(),
        }
    }
}

impl Vault {
    /// Create connection to a directory of markdown files
    pub fn connect(
        base: &str,
        pattern: Option<&str>,
        name: Option<&str>,
        options: Option<MatchOptions>,
    ) -> Self {
        let _pattern = pattern.unwrap_or("*.md");
        let _name = name.unwrap_or("");
        let _options = options.unwrap_or(MatchOptions::new());
        // TODO: we should expand the path first
        let _path = PathBuf::from_str(base).expect("Could not create vault from path");

        Self {
            name: _name.to_string(),
            base: _path,
            pattern: _pattern.to_string(),
            options: _options,
            index: BTreeMap::new(),
        }
    }

    /// Return the files that match the glob pattern
    pub fn get_files(&self) -> Result<Paths, PatternError> {
        let path = self.base.join(self.pattern.clone()).display().to_string();
        glob_with(path.as_str(), self.options)
    }

    /// Confirm if this vault has a root.md file
    pub fn has_root(&self) -> bool {
        let root_file = "root.md";
        self.base.join(root_file).try_exists().unwrap()
    }

    /// either get the id from root.md or create a new one
    pub fn get_root_id(&self) -> String {
        let root_file = self.base.join("root.md");
        let id = match root_file.try_exists() {
            Ok(true) => {
                let content = fs::read_to_string(root_file).unwrap();
                let re = Regex::new(r"id = (\w+)$").unwrap();
                let matches = re.captures(&content).unwrap();
                let id = matches.get(1).map_or("", |m| m.as_str());

                if id.len() > 0 {
                    &id.to_string()
                } else {
                    &Id::default().to_string()
                }
            }
            Ok(false) => &Id::default().to_string(),
            Err(_e) => &Id::default().to_string(),
        };
        id.clone()
    }

    /// Add a file to the vault at the given relative path
    pub fn add(&self, rpath: &str, content: Option<&str>) -> Result<(), std::io::Error> {
        let full_path = self.base.join(rpath);

        match fs::File::create_new(full_path) {
            Ok(file) => {
                //TODO: Logging ?
                println!("Writing to file {:?}", file);
                if content.is_some() {
                    //TODO: Add the content to the file
                } else {
                    //TODO Create a new file with the default info
                };
                Ok(())
            }
            Err(e) => return Err(e),
        }
    }

    /// Return the relative path of the file compared to the vault base directory
    pub fn rel_path(&self, path: &Path) -> Result<RelativePathBuf, FromPathError> {
        let base = match RelativePath::from_path(self.base.as_path()) {
            Ok(b) => b,
            Err(e) => return Err(e),
        };
        let rel = match RelativePath::from_path(path) {
            Ok(p) => p,
            Err(e) => return Err(e),
        };
        Ok(base.relative(rel))
    }

    /// update the contents of the given file
    pub fn update(&self, rpath: &str, content: &str) -> Result<(), DataStoreError> {
        todo!("Update the contents of {:?} with {:?}", rpath, content);
    }

    /// remove (delete) a file from the vault
    pub fn remove(&self, rpath: &str) -> Result<(), DataStoreError> {
        todo!("remove the given file from the vault {:?}", rpath);
    }
}
// region: Tests

#[cfg(test)]
mod test {
    use super::Vault;
    use std::{
        env,
        path::{Path, PathBuf},
        str::FromStr,
    };

    fn get_data_dir() -> PathBuf {
        let workspace = Path::new(env!("CARGO_MANIFEST_DIR"));
        workspace.join("test/data")
    }

    #[test]
    fn new_vault_with_no_params() {
        let out_dir = env!("TEMP");
        let expected = PathBuf::from_str(out_dir).expect("Could not create path");
        let v = Vault::connect(&out_dir, Some("*.md"), Some("default"), None);
        assert_eq!(expected, v.base);
    }

    #[test]
    fn new_vault_get_files() {
        let binding = get_data_dir();
        let data_dir = binding.display();
        let v = Vault::connect(data_dir.to_string().as_str(), None, Some("test_data"), None);
        let i = v.get_files().unwrap();
        assert_eq!(3, i.count(), "There should be {} files", 3);
        assert_eq!("test_data", v.name);
    }

    #[test]
    fn vault_relative_path() {
        let workspace = Path::new(env!("CARGO_MANIFEST_DIR"));
        let data_dir = workspace.join("test/data");
        let v = Vault::connect(
            workspace.display().to_string().as_str(),
            None,
            Some("data"),
            None,
        );
        let rel = "test/data";
        let rel_path = v.rel_path(&data_dir).unwrap();
        assert_eq!(rel, rel_path.as_str())
    }
}
// endregion Tests
