// region: imports

//- mods
mod cb;
mod file;
mod fm;
mod h;

//- stdlib
use std::{fs::read_to_string, io::Error, path::PathBuf};

//- crates
use markdown_it::{
    parser::inline::Text, plugins::cmark::block::heading::ATXHeading, MarkdownIt, Node,
};
use markdown_it_front_matter::FrontMatter;
use markdown_it_tasklist::TodoCheckbox;

use log::trace;
use pathdiff::diff_paths;

//- local
pub use crate::parser::{cb::CheckboxData, file::FileData, h::HeadingData};

// endregion imports

pub struct Parser {
    parser : MarkdownIt,
}

impl Parser {
    /// Create a new instance of the markdown parser.  Loads required plugins.
    pub fn new() -> Self {
        let mut parser = markdown_it::MarkdownIt::new();
        markdown_it::plugins::cmark::add(&mut parser);
        markdown_it_front_matter::add(&mut parser);
        markdown_it_tasklist::add(&mut parser);
        Self { parser }
    }

    /// Parses the given file
    pub fn parse(&self, path : &PathBuf, base : &PathBuf) -> Result<FileData, Error> {
        // Read in the file content
        let content : String = read_to_string(path).expect("Could not read markdown file");
        // Convert the content to a Markdown AST
        let ast : Node = self.parser.parse(content.as_str());
        let mut file_data = FileData::new();

        let folders = diff_paths(path, base).unwrap();
        file_data.path = path.clone();
        let fname = path.file_stem().unwrap().to_string_lossy().to_string();
        let parts : Vec<String> = fname.split(".").map(str::to_string).collect();

        file_data.domain = parts[0].clone();
        file_data.hierarchy = parts;

        // Fill in the the data into a FileData
        ast.walk(|node, _depth| {
            if let Some(fm) = node.cast::<FrontMatter>() {
                let data : serde_yml::Value = serde_yml::from_str(fm.content.as_str())
                    .expect("Could not transform data in markdown frontmatter");
                file_data.add_front_matter(data);
            } else if let Some(hd) = node.cast::<ATXHeading>() {
                if let Some(h_title) = node.children[0].cast::<Text>() {
                    let h_title = h_title.content.as_str();
                    file_data.add_heading(h_title, hd.level);
                }
            }
            if let Some(cb) = node.cast::<TodoCheckbox>() {
                if let Some(cb_title) = node.children[0].cast::<Text>() {
                    let cb_title = cb_title.content.as_str();
                    file_data.add_checkbox(cb_title, Some(cb.checked));
                }
            }
        });
        trace!("File Data\n{file_data:#?}");
        Ok(file_data)
    }
}

// endregion Parser

// region: Tests
// endregion Tests
