use serde_derive::Deserialize;

// region: FileFrontMatter
#[allow(unused)]
#[derive(Debug, Deserialize, PartialEq, Eq, Clone)]
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
