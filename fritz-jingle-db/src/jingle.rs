use serde::{ Serialize, Deserialize };

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Jingle {
    pub name: String,
    pub url: String,
    pub date_time: String,
    pub file_path: String
}
