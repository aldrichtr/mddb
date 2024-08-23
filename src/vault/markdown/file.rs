use std::path::PathBuf;

use log::trace;

use crate::parser::{CheckboxData, HeadingData};

#[derive(Debug, PartialEq, Eq, Clone, Default)]
pub struct FileData {
    pub path : PathBuf,
    pub fname : String,
    pub domain : String,
    pub hierarchy : Vec<String>,
    pub memberof : String,
    pub front_matter : serde_yml::Value,
    pub headings : Vec<HeadingData>,
    pub check_boxes : Vec<CheckboxData>,
}

impl FileData {
    pub fn new() -> Self {
        FileData::default()
    }

    pub fn add_front_matter(&mut self, fm : serde_yml::Value) {
        trace!("Adding frontmatter {fm:#?}");
        self.front_matter = fm;
    }

    pub fn add_heading(&mut self, title : &str, level : u8) {
        let hd = HeadingData::new(title, level);
        self.headings.push(hd);
    }

    pub fn add_checkbox(&mut self, title : &str, checked : Option<bool>) {
        let cb = CheckboxData::new(title, checked);
        self.check_boxes.push(cb);
    }
}
