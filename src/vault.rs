// region: imports
//- stdlib
use std::{
    default::Default,
    fs,
    path::{Path, PathBuf},
};

//- crates
use glob::{glob_with, MatchOptions, Paths, PatternError};
use log::{debug, error, info, trace, warn};
use pathdiff::diff_paths;
use tree_ds::prelude::{Node, NodeRemovalStrategy::RemoveNodeAndChildren, Tree};
use normalize_path::NormalizePath;
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
        base: PathBuf,
        pattern: Option<&str>,
        name: Option<&str>,
        options: Option<MatchOptions>,
    ) -> Result<Self, DataStoreError> {
        // either use the arguments or the defaults
        let def = &Vault::default();
        let _pattern = pattern.unwrap_or(def.pattern.as_str());
        let mut _name = name.unwrap_or(def.name.as_str());
        let _options = options.unwrap_or(def.options);
        // TODO: we should expand the path first, and validate it is available here
        //       perhaps a validate_path function?
        match base.try_exists() {
            Ok(true) => {
                info!("using {:?} as base directory", base.display());
            }
            Ok(false) => {
                error!("{base:?} does not exist");
                return Err(DataStoreError::VaultReadError {
                    path: base,
                    msg: String::from("Path does not exist"),
                });
            }
            Err(e) => {
                error!("{base:?} could not be accessed");
                return Err(DataStoreError::VaultReadError {
                    path: base,
                    msg: e.to_string(),
                });
            }
        }
        if _name.is_empty() {
            debug!("No name was given.  Using directory name");
            let dir_name = base.components().last().unwrap().as_os_str();
            _name = dir_name.to_str().unwrap();
            debug!("- name is now {_name:?}");
        }
        // now create a vault object from the given args
        let mut s = Self {
            name: _name.to_string(),
            base: base.canonicalize().unwrap().clone(),
            pattern: _pattern.to_string(),
            options: _options,
            tree: Tree::new(Some(_name)),
        };
        debug!("Created new vault {:#?}", s);
        // Call load to populate the internal tree
        debug!("Loading markdown files into vault");
        match s.load() {
            Ok(num_files) => {
                debug!("- Loading completed");
                // if there wasn't an error loading the files, and there was at least one file parsed, return the Vault
                if num_files > 0 {
                    debug!("- parsed {num_files:?} markdown files");
                    Ok(s)
                } else {
                    error!("- No markdown files were found in vault");
                    Err(DataStoreError::EmptyVaultError { path: base })
                }
            }
            Err(e) => {
                error!("There was an error loading files");
                Err(e)
            }
        }
    }


    fn convert_to_glob(&self) -> String {
        debug!("Converting to glob pattern");
        let path = self.base.display().to_string();
        trace!("base converted to {path:?}");
        let mut path = shellexpand::full(path.as_str()).unwrap().to_string();
        trace!("base expanded to {path:?}");
        path.push_str(std::path::MAIN_SEPARATOR_STR);
        trace!("Added the separator {path:?}");
        path.push_str(self.pattern.as_str());
        debug!("final glob pattern is {path:?}");
        path
    }

    /// Return the files that match the glob pattern
    pub fn get_files(&self) -> Result<Paths, DataStoreError> {
        debug!("Getting files in the vault");

        let pattern = self.convert_to_glob();
        debug!("File pattern is {:?}", pattern);
        match glob_with(pattern.as_str(), self.options) {
            Ok(mut p) => {
                let count = p.by_ref().count();
                debug!("Pattern returned {:?} files", count);
                return Ok(p);
            }
            Err(e) => {
                error!("Glob pattern error {e:?}");
                return Err(DataStoreError::EmptyVaultError {
                    path: self.base.clone(),
                });
            }
        }
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
        trace!("---------------------------- Initialize the AST -----------------------------------------------");
        // if there is already a tree, remove it first
        trace!("Checking to see if there is already an AST");
        if let Some(root) = self.tree.get_root_node() {
            trace!("- An AST exists");
            let id = &root.get_node_id();
            debug!("Attempting to remove AST with root {id:?}");
            match self.tree.remove_node(id, RemoveNodeAndChildren) {
                Ok(_) => {
                    info!("Cleared previous AST")
                }
                Err(e) => {
                    warn!("could not remove previous AST: {e:?}");
                    return Err(DataStoreError::AstError);
                }
            }
        } else {
            debug!("No AST found");
        }

        // If there is a root.md file, use it as the basis for our tree
        debug!("Looking for a root file");
        if let Some(root_file) = self.get_root_file() {
            debug!("Found root file {root_file:?}");
            let parser = Parser::new();
            debug!("Parsing root file");
            if let Ok(fd) = parser.parse(&root_file) {
                debug!("Parsing successful");
                let mut id = fd.front_matter.id.clone();
                if id.is_empty() {
                    debug!("Root file did not contain an id field");
                    id = Id::default().to_string();
                };
                debug!("Found id {id:?} in front matter");
                let n = Node::new(id, Some(fd));
                // we parsed the root file, so create the root node
                match self.tree.add_node(n, None) {
                    Ok(root) => {
                        debug!("Adding root node to AST");
                        return Ok(root);
                    }
                    Err(_) => {
                        error!("Could not add root node to AST");
                        return Err(DataStoreError::AstError);
                    }
                }
            } else {
                error!("Could not parse {root_file:?}");
                Err(DataStoreError::AstError)
            }
        } else {
            debug!("No root file found in {:?}", self.base);
            let id = Id::default().to_string();
            let fd = FileData::default();
            let n = Node::new(id, Some(fd));
            // There is no root file, that's ok, just create the root node with a default filedata
            if let Ok(root) = self.tree.add_node(n, None) {
                debug!("Add generated root node to AST. No root file");
                Ok(root)
            } else {
                error!("Could not add generated root node to AST");
                Err(DataStoreError::AstError)
            }
        }
    }

    /// parse all documents in the vault to create a tree.
    fn load(&mut self) -> Result<i64, DataStoreError> {
        debug!("------------------------ Load ---------------------------------------");
        // keep track of how many files we have loaded
        let mut counter: i64 = 0;
        let parser = Parser::new();
        // this will get the id from the file or create a new one if it doesn't
        // exist
        if let Ok(root) = self.init_tree() {
            debug!("Gathering files using {:?}", self.pattern);
            for file in self.get_files().expect("Could not get files in vault") {
                debug!("Reading {:?}", file);
                match file {
                    Ok(file) => {
                        if file.file_name().unwrap().to_os_string() == PathBuf::from("root.md") {
                            debug!("file {counter:?} is the root file, skipping parse");
                            counter += 1;
                            continue;
                        } else {
                            debug!("Parsing {file:?}");
                            if let Ok(fd) = parser.parse(&file) {
                                let mut id = fd.front_matter.id.clone();
                                if id.is_empty() {
                                    debug!("No id found in file frontmatter, generating new");
                                    id = Id::default().to_string();
                                }
                                debug!("Creating new AST node for {file:?}");
                                let n = Node::new(id, Some(fd));
                                if let Ok(child) = self.tree.add_node(n, Some(&root)) {
                                    debug!("Added child with id {child:?}");
                                    counter += 1
                                } else {
                                    warn!("Could not add child");
                                    ()
                                }
                            } else {
                                error!("There was an error during parsing file {file:?}");
                                ()
                            };
                        }
                    }
                    Err(e) => {
                        error!("Error getting files {:?}", e);
                    }
                }
            }
            debug!("Completed parsing {counter:?} files");
            Ok(counter)
        } else {
            error!("Failed to initialize AST");
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
    use k9::assert_equal;
    use std::{
        env,
        path::{Path, PathBuf},
    };
    use stdext::function_name;
    /// Utility functions for the tests.
    mod util {
        use log::debug;
        use std::path::PathBuf;
        /// Returns the root of the workspace
        pub(super) fn get_workspace_dir() -> PathBuf {
            let ws = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
            debug!("Workspace dir is {:?}", ws.display());
            ws
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
            debug!("Data dir is {:?}", workspace.display());
            workspace
        }
    }

    #[test_log::test]
    fn new_vault_with_no_params() {
        let out_dir = PathBuf::from(env!("TEMP"));
        let v = Vault::connect(out_dir.clone(), None, None, None).unwrap();
        assert_equal!(out_dir, v.base);
        assert_equal!("*.md", v.pattern);
        assert_equal!("Temp", v.name);
        assert_equal!(glob::MatchOptions::new(), v.options);
    }

    #[test_log::test]
    fn new_vault_with_params() {
        let out_dir = PathBuf::from(env!("TEMP"));
        let v = Vault::connect(out_dir.clone(), Some("*.md"), Some("default"), None).unwrap();
        assert_equal!(out_dir, v.base);
    }

    #[test_log::test]
    fn new_vault_get_files() {
        let data_dir = util::get_data_dir(Some(function_name!()));
        let v = Vault::connect(data_dir.clone(), None, Some("test_data"), None).unwrap();
        let i = v.get_files().unwrap();
        assert_equal!(3, i.count(), "There should be {} files", 3);
        assert_equal!("test_data", v.name);
    }

    #[test_log::test]
    fn vault_relative_path() {
        let workspace = util::get_workspace_dir();
        let data_dir = util::get_data_dir(None);
        let v = Vault::connect(workspace.clone(), None, Some("data"), None).unwrap();
        let rel = Path::new("test/data").to_path_buf();
        let rel_path = v.rel_path(&data_dir).unwrap_or(PathBuf::new());
        assert_equal!(rel, rel_path);
    }
    #[test_log::test]
    fn vault_has_root_file() {
        let data_dir = util::get_data_dir(Some(function_name!()));
        let v = Vault::connect(data_dir.clone(), None, None, None).unwrap();
        let root_file = v.get_root_file();
        assert_equal!(true, root_file.is_some());
    }

    #[test_log::test]
    fn get_root_id_with_root_file() {
        let id = String::from("7csf5wyep1i96o3x35oalg5");
        let data_dir = util::get_data_dir(Some(function_name!()));
        let v = Vault::connect(data_dir.clone(), None, None, None).unwrap();

        let t = v.get_tree();
        let root = match t.get_root_node() {
            Some(root) => root.get_node_id(),
            None => String::from("-"),
        };
        assert_equal!(id, root);
    }
}
// endregion Tests
