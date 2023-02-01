use serde::{Serialize, Deserialize};

const EMPTY: &str = "";

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Readme {
    // user of GitHub/GitLab
    #[serde(default = "get_empty")]
    pub user: String,
    // project of GitHub/GitLab
    // user/project
    #[serde(default = "get_empty")]
    pub project: String,
    #[serde(default = "get_empty")]
    pub title: String,
    #[serde(default = "get_empty")]
    pub license: String,
    #[serde(default = "get_empty")]
    pub icon: String,
    #[serde(default = "get_empty")]
    pub homepage: String,
}

fn get_empty() -> String{
    return EMPTY.to_string();
}

impl Readme {
    pub fn from_file(file: &str) -> Self{

    }
}
