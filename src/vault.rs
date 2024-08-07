// region: imports
//- stdlib
use std::{
    default::Default,
    fs,
    path::{Path, PathBuf},
    str::FromStr,
};

//- crates
use glob::{glob_with, MatchOptions, Paths, PatternError};
use pathdiff::diff_paths;
use tree_ds::prelude::{Node, Tree,NodeRemovalStrategy::RemoveNodeAndChildren};
use log::{debug, info, warn};
//- local
use crate::{
    id::Id,
    parser::{FileData, Parser},
};

use crate::error::DataStoreError;

// endregion imports

#[derive(Debug)]
pub struct Vault {
    pub name: String,
    pub base: PathBuf,
    pub pattern: String,
    pub options: MatchOptions,
    tree: Tree<String, FileData>,
}

impl Default for Vault {
    fn default() -> Self {
        Self {
            name: String::from(""),
            base: PathBuf::new(),
            pattern: String::from("*.md"),
            options: MatchOptions::new(),
            tree: Tree::new(None),
        }
    }
}

impl Vault {
    /// Create connection to a directory of markdown files.  Calling this function causes the markdown files to be
    /// parsed and added to the AST
    pub fn connect(
        base: &str,
        pattern: Option<&str>,
        name: Option<&str>,
        options: Option<MatchOptions>,
    ) -> Result<Self, DataStoreError> {
        // either use the arguments or the defaults
        let def = &Vault::default();
        let _pattern = pattern.unwrap_or(def.pattern.as_str());
        let _name = name.unwrap_or(def.name.as_str());
        let _options = options.unwrap_or(def.options);
        // TODO: we should expand the path first, and validate it is available here
        //       perhaps a validate_path function?
        let _path = PathBuf::from_str(base).expect("Could not create vault from path");

        // TODO: We probably want to validate the name too
        // If there was no name given, then take the directory name
        if _name.is_empty() {
            let _name = _path.components().last().unwrap();
        }
        // now create a vault object from the given args
        let mut s = Self {
            name: _name.to_string(),
            base: _path.clone(),
            pattern: _pattern.to_string(),
            options: _options,
            tree: Tree::new(Some(_name)),
        };
        // Call load to populate the internal tree
        match s.load() {
            Ok(num_files) => {
                // if there wasn't an error loading the files, and there was at least one file parsed, return the Vault
                if num_files > 0 {
                    Ok(s)
                } else {
                    Err(DataStoreError::EmptyVaultError { path: _path })
                }
            },
            Err(e) => {
                Err(e)
            }
        }
    }

    /// Return the files that match the glob pattern
    pub fn get_files(&self) -> Result<Paths, PatternError> {
        let path = self.base.join(self.pattern.clone()).display().to_string();
        glob_with(path.as_str(), self.options)
    }

    /// Returns either a PathBuf or None if it doesn't exist
    fn get_root_file(&self) -> Option<PathBuf> {
        let root_file = "root.md";
        let root_file = self.base.join(root_file);
        if root_file.exists() {
            Some(root_file)
        } else {
            None
        }
    }

    /// either get the id from root.md or create a new one
    fn get_root_id(&self) -> Option<String> {
        if let Some(n) = self.tree.get_root_node() {
            Some(n.get_node_id())
        } else {
            None
        }

    }

    // Initialize the root element with either the root.md file, or a blank root
    fn init_tree(&mut self) -> Result<String, DataStoreError> {
        // if there is already a tree, remove it first
        if let Some(root) = self.tree.get_root_node() {
            let id = &root.get_node_id();
                match self.tree.remove_node(id, RemoveNodeAndChildren) {
                    Ok(_) => { info!("Cleared previous AST")},
                    Err(e) => {
                        warn!("could not remove previous AST: {e:?}");
                        return Err(DataStoreError::AstError)
                    }
                }
        } else {}

        // If there is a root.md file, use it as the basis for our tree
        if let Some(root_file) = self.get_root_file() {
            let parser = Parser::new();
            if let Ok(fd) = parser.parse(&root_file) {
                let id = fd.front_matter.id.clone();
                let n = Node::new(id, Some(fd));
                // we parsed the root file, so create the root node
                match self.tree.add_node(n, None) {
                    Ok(root) => return Ok(root),
                    Err(_) => return Err(DataStoreError::AstError)
                }
            } else {
                Err(DataStoreError::AstError)
            }
        } else {
            let id = Id::default().to_string();
            let fd = FileData::default();
            let n = Node::new(id, Some(fd));
            // There is no root file, that's ok, just create the root node with a default filedata
            if let Ok(root) = self.tree.add_node(n, None) {
                println!("Add root with no file");
                Ok(root)
            } else {
                println!("Could not add root node with no file");
                Err(DataStoreError::AstError)
            }
            }

    }

    /// parse all documents in the vault to create a tree.
    fn load(&mut self) -> Result<i64, DataStoreError> {
        // keep track of how many files we have loaded
        let mut counter: i64 = 0;
        let parser = Parser::new();
        // this will get the id from the file or create a new one if it doesn't
        // exist
        if let Ok(root) = self.init_tree() {
            for file in self.get_files().expect("Could not get files in vault") {
                match file {
                    Ok(file) => {
                        if file.file_name().unwrap().to_os_string() == PathBuf::from("root.md") {
                            continue;
                        } else {
                            if let Ok(fd) = parser.parse(&file) {
                                let id = fd.front_matter.id.clone();
                                let n = Node::new(id, Some(fd));
                                if let Ok(child) = self.tree.add_node(n, Some(&root)) {
                                    debug!("Added child with id {child:?}");
                                    counter += 1
                                } else {
                                    ()
                                }
                            } else {
                                // There was an error during parsing
                                ()
                            };
                        }
                    }
                    Err(e) => {
                        println!("Error getting files {:?}", e);
                    }
                }
            }
            Ok(counter)
        } else {
            Err(DataStoreError::AstError)
        }
    }

    /// Add a file to the vault at the given relative path
    pub fn add(&self, rpath: &str, content: Option<&str>) -> Result<(), std::io::Error> {
        let full_path = self.base.join(rpath);

        match fs::File::create_new(full_path) {
            Ok(file) => {
                // TODO: Logging ?
                println!("Writing to file {:?}", file);
                if content.is_some() {
                    todo!("Add the content to the file, write the file");
                    // TODO: Add the content to the file
                } else {
                    // TODO Create a new file with the default info
                };
                Ok(())
            }
            Err(e) => return Err(e),
        }
    }

    /// Return the relative path of the file compared to the vault base
    /// directory
    pub fn rel_path(&self, path: &Path) -> Option<PathBuf> {
        println!("Checking relative path from base {:#?}", self.base);
        let base = self.base.clone();
        diff_paths(path, base)
    }

    /// update the contents of the given file
    pub fn update(&self, rpath: &str, content: &str) -> Result<(), DataStoreError> {
        todo!("Update the contents of {:?} with {:?}", rpath, content);
    }

    /// remove (delete) a file from the vault
    pub fn remove(&self, rpath: &str) -> Result<(), DataStoreError> {
        todo!("remove the given file from the vault {:?}", rpath);
    }

    pub fn get_tree(&self) -> &Tree<String, FileData> {
        &self.tree
    }
}
// region: Tests

#[cfg(test)]
mod tests {
    use super::Vault;
    use std::{
        env,
        path::{Path, PathBuf},
        str::FromStr,
    };
    use stdext::function_name;

    /// Utility functions for the tests.
    mod util {
        use std::path::PathBuf;
        /// Returns the root of the workspace
        pub(super) fn get_workspace_dir() -> PathBuf {
            PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        }

        /// The test data directory (`test/data`) is where all of the data for
        /// the different tests are. if the function name (`fn_name`) is
        /// given, we remove everything except the actual name, and append that
        /// to the data directory.  If the directory does not exist, it is
        /// created.
        pub(super) fn get_data_dir(fn_name: Option<&str>) -> PathBuf {
            let mut workspace = get_workspace_dir();
            workspace = workspace.join("test/data");
            if fn_name.is_some() {
                let fn_name = fn_name.expect("Could not get name");
                workspace = workspace.join(fn_name.split("::").last().unwrap().to_string());
            };
            if !workspace.exists() {
                std::fs::create_dir_all(workspace.clone())
                    .expect("Could not create path {workspace}");
            }
            workspace.canonicalize().unwrap()
        }
    }

    #[test]
    fn new_vault_with_no_params() {
        let out_dir = env!("TEMP");
        let expected = PathBuf::from_str(out_dir).expect("Could not create path");
        let v = Vault::connect(&out_dir, Some("*.md"), Some("default"), None).unwrap();
        assert_eq!(expected, v.base);
    }

    #[test]
    fn new_vault_get_files() {
        let name = util::get_data_dir(Some(function_name!()));
        let data_dir = name.display();
        let v =
            Vault::connect(data_dir.to_string().as_str(), None, Some("test_data"), None).unwrap();
        let i = v.get_files().unwrap();
        assert_eq!(3, i.count(), "There should be {} files", 3);
        assert_eq!("test_data", v.name);
    }

    #[test]
    fn vault_relative_path() {
        let workspace = util::get_workspace_dir();
        let data_dir = util::get_data_dir(None);
        let v = Vault::connect(
            workspace.display().to_string().as_str(),
            None,
            Some("data"),
            None,
        )
        .unwrap();
        let rel = Path::new("test/data").to_path_buf();
        let rel_path = v.rel_path(&data_dir).unwrap_or(PathBuf::new());
        assert_eq!(rel, rel_path);
    }
    #[test]
    fn vault_has_root_file() {}

    #[test]
    fn can_connect_to_vault() {
        let expected = String::from("~/dendron/vaults/journal/notes");
        let v = Vault::connect(
            "~/dendron/vaults/journal/notes",
            Some("*.md"),
            Some("journal"),
            None,
        )
        .expect("There was an error loading vault");
        assert_eq!(expected, v.base.display().to_string());
    }

    #[test]
    fn get_root_id_with_root_file() {
        let data_dir = util::get_data_dir(Some(function_name!())).display();
        let v = Vault::connect(data_dir,None, None, None);
    }
}
// endregion Tests
