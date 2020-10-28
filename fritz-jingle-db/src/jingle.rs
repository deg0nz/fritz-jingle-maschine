use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Jingle {
    pub name: String,
    pub url: String,
    pub date_time: String,
    pub file_path: String,
}

impl PartialEq for Jingle {
    fn eq(&self, other: &Self) -> bool {
        self.url == other.url
    }
}
