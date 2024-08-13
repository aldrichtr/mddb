use std::fmt::{Display, Formatter, Result};

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct HeadingData {
    title : String,
    level : u8,
}

impl Display for HeadingData {
    fn fmt(&self, f : &mut Formatter) -> Result {
        let mut h = String::new();
        for _i in 0..self.level {
            h.push('#');
        }
        write!(f, "{} {}", h, self.title)
    }
}

impl HeadingData {
    pub fn new(title : &str, level : u8) -> Self {
        Self {
            title : title.to_string(),
            level,
        }
    }
}
