use serde::Deserialize;
use serde_json;
use std::{error::Error, io};
use std::io::prelude::*;
use std::fs::File;
use rand::Rng;

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct JingleDbObject {
    pub name: String,
    pub url: String,
    pub date_time: String,
    pub file_path: String
}
pub struct JinglesDb {
    db: Vec<JingleDbObject>
}

impl JinglesDb {
    fn new(jsonFilePath: String) -> Result<Self, Box<dyn Error>> {
        let mut f = File::open("foo.txt")?;
        let mut buffer = String::new();
        f.read_to_string(&mut buffer)?;

        let db: Vec<JingleDbObject> = serde_json::from_str(buffer.as_str())?;

        Ok(Self {
            db
        })
    }

    fn get_random_entry(&self) -> JingleDbObject{
        let mut rng = rand::thread_rng();
        let index = rng.gen_range(0, self.db.len()-1);

        self.db[index].clone()
    }
}