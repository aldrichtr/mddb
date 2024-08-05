
use crate::parser::*;


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

    pub fn add_checkbox(&mut self, title : &str, checked : Option<bool>) {
        let cb = CheckboxData::new(title, checked);
        self.check_boxes.push(cb);
    }
}
