pub mod jingle;

use serde_json;
use std::{error::Error, path::PathBuf};
use std::io::prelude::*;
use std::fs::File;
use rand::Rng;
use jingle::Jingle;
use eyre::Result;

pub struct JinglesDb {
    db: Vec<Jingle>,
    path: PathBuf
}

impl JinglesDb {
    pub fn new(json_file_path: PathBuf) -> Result<Self> {
        let db: Vec<Jingle> = Vec::new();
        let path = json_file_path;

        if let Err(_err) = File::open(&path) {
            File::create(&path)?;
        }

        Ok(Self {
            db,
            path
        })
    }

    pub fn load(&mut self) -> Result<()> {
        let mut f = File::open(&self.path)?;
        let mut buffer = String::new();
        f.read_to_string(&mut buffer)?;
        self.db = serde_json::from_str(buffer.as_str())?;
        Ok(())
    }

    pub fn save(&self) -> Result<()>{
        let f = File::open(&self.path)?;
        serde_json::to_writer(f, &self.db)?;

        Ok(())
    }

    pub fn get_random_jingle(&self) -> Jingle {
        let mut rng = rand::thread_rng();
        let index = rng.gen_range(0, self.db.len()-1);
        let db_object = self.db[index].clone();
        println!("Selected jingle no {} with name: {}", index, db_object.name);
        db_object
    }

    pub fn add_jingle(&mut self, jingle: Jingle) {
        self.db.push(jingle);
    }

    pub fn contains(&self, jingle: &Jingle) -> bool {
        self.db.contains(jingle)
    }
}