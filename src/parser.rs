// region: imports

//- mods
mod file;
mod fm;
mod h;
mod cb;

//- stdlib
use std::{
    fs::read_to_string,
    path::PathBuf,
};


//- crates
use markdown_it::{
    parser::inline::Text, plugins::cmark::block::heading::ATXHeading, MarkdownIt, Node,
};
use markdown_it_front_matter::FrontMatter;
use markdown_it_tasklist::TodoCheckbox;

use serde_yml;

//- local
use crate::parser::{
    file::FileData,
    fm::FileFrontMatter,
    h::HeadingData,
    cb::CheckboxData
};

// endregion imports

pub struct Parser {
    parser : MarkdownIt,
}

impl Parser {
    pub fn new() -> Self {
        let mut parser = markdown_it::MarkdownIt::new();
        markdown_it::plugins::cmark::add(&mut parser);
        markdown_it_front_matter::add(&mut parser);
        markdown_it_tasklist::add(&mut parser);
        Self { parser }
    }

    pub fn parse(&mut self, path : &PathBuf) {
        // Read in the file content
        let content : String = read_to_string(path).expect("Could not read markdown file");
        // Convert the content to a Markdown AST
        let ast : Node = self.parser.parse(content.as_str());
        let mut file_data = FileData::new();
        // Fill in the the data into a FileData
        ast.walk(|node, _depth| {
            if let Some(fm) = node.cast::<FrontMatter>() {
                let data : FileFrontMatter = serde_yml::from_str(fm.content.as_str())
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
    }
}

// endregion Parser

// region: Tests
// endregion Tests
