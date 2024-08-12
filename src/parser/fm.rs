use serde_derive::Deserialize;

// region: FileFrontMatter
#[allow(unused)]
#[derive(Debug, Deserialize, PartialEq, Eq, Clone)]
#[derive(Default)]
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


// endregion FileFrontMatter
