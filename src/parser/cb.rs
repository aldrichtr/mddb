
#[derive(Debug)]
pub struct CheckboxData {
    pub title : String,
    pub checked : bool,
}

impl Default for CheckboxData {
    fn default() -> Self {
        Self {
            title: String::from(""),
            checked: false
        }
    }
}

impl CheckboxData {
    pub fn new(title : &str, check : Option<bool>) -> Self {
        Self {
            title : title.to_string(),
            checked : check.unwrap_or(false),
        }
    }

    pub fn check(&mut self) {
        self.checked = true
    }


    pub fn uncheck(&mut self) {
        self.checked = false
    }

    pub fn set_title(&mut self, title: &str) {
        self.title = title.to_string()
    }
}


#[cfg(test)]
mod test {
    use super::CheckboxData;

    #[test]
    pub fn new_checkbox_with_title() {
        let title = "A test checkbox with default value";
        let cb = CheckboxData::new(title, None);
        assert_eq!(title, cb.title);
        assert_eq!(false, cb.checked);
    }
}
