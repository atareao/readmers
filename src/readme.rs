use serde::{Serialize, Deserialize};

const EMPTY: &str = "";
const FALSE: bool = false;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Readme {
    #[serde(default = "get_empty")]
    pub name: String,
    #[serde(default = "get_empty")]
    pub title: String,
    #[serde(default = "get_empty")]
    pub project: String,
    #[serde(default = "get_empty")]
    pub license: String,
    #[serde(default = "get_empty")]
    pub icon: String,
    #[serde(default = "get_empty")]
    pub homepage: String,
    #[serde(default = "get_false")]
    pub license_badge: bool,
    #[serde(default = "get_false")]
    pub contributors_badge: bool,
    #[serde(default = "get_false")]
    pub lastcommit_badge: bool,
    #[serde(default = "get_false")]
    pub codefactor_badge: bool,
}

fn get_empty() -> String{
    return EMPTY.to_string();
}

fn get_false() -> bool{
    return FALSE;
}

impl Readme {
    // add code here
    pub fn from_file(file: &str) -> Self{


    }
}
