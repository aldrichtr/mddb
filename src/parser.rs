// region: imports
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
use serde_derive::Deserialize;
use serde_yml;

use tree_ds::prelude::*;

//- local

// endregion imports

// region: FileFrontMatter
#[allow(unused)]
#[derive(Debug, Deserialize, PartialEq)]
pub struct FileFrontMatter {
    pub id : String,
    pub title : String,
    pub desc : String,
    pub updated : String,
    pub created : String,
    #[serde(default)]
    pub tags : Vec<String>,
    #[serde(default)]
    pub status : String,
    #[serde(default)]
    pub priority : String,
    #[serde(default)]
    pub owner : String,
}

impl Default for FileFrontMatter {
    fn default() -> Self {
        Self {
            id : String::from(""),
            title : String::from(""),
            desc : String::from(""),
            tags : Vec::new(),
            updated : String::from(""),
            created : String::from(""),
            status : String::from(""),
            priority : String::from(""),
            owner : String::from(""),
        }
    }
}

// endregion FileFrontMatter

// region: CheckboxData
#[derive(Debug)]
pub struct CheckboxData {
    title : String,
    checked : bool,
}

impl CheckboxData {
    pub fn new(ttl : &str, chk : bool) -> Self {
        Self {
            title : ttl.to_string(),
            checked : chk,
        }
    }
}

// endregion CheckboxData

// region: HeadingData
#[derive(Debug)]
pub struct HeadingData {
    title : String,
    level : u8,
}

impl HeadingData {
    pub fn new(ttle : &str, lvl : u8) -> Self {
        Self {
            title : ttle.to_string(),
            level : lvl,
        }
    }

    pub fn to_string(&self) -> String {
        let mut h = String::new();
        for _i in 0..self.level {
            h.push('#');
        }
        let st = String::from(format!("{} {}", h, self.title));
        st.clone()
    }
}
// endregion HeadingData

// region: FileData

#[derive(Debug)]
pub struct FileData {
    pub front_matter : FileFrontMatter,
    pub headings : Vec<HeadingData>,
    pub check_boxes : Vec<CheckboxData>,
}

impl Default for FileData {
    fn default() -> Self {
        Self {
            front_matter : FileFrontMatter::default(),
            headings : Vec::new(),
            check_boxes : Vec::new(),
        }
    }
}

impl FileData {
    pub fn new() -> Self {
        FileData::default()
    }

    pub fn add_front_matter(&mut self, fm : FileFrontMatter) {
        self.front_matter = fm;
    }

    pub fn add_heading(&mut self, title : &str, level : u8) {
        let hd = HeadingData::new(title, level);
        self.headings.push(hd);
    }

    pub fn add_checkbox(&mut self, title : &str, checked : bool) {
        let cb = CheckboxData::new(title, checked);
        self.check_boxes.push(cb);
    }
}

// endregion FileData


// region: Parser
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
                    file_data.add_checkbox(cb_title, cb.checked);
                }
            }
        });
        Task::from(file_data)
    }
}

// endregion Parser

// region: Tests
// endregion Tests
