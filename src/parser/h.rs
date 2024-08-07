#[derive(Debug, PartialEq, Eq, Clone)]
pub struct HeadingData {
    title : String,
    level : u8,
}

impl HeadingData {
    pub fn new(title : &str, level : u8) -> Self {
        Self {
            title : title.to_string(),
            level : level,
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
