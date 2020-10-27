pub mod jingle;

use eyre::Result;
use jingle::Jingle;
use rand::Rng;
use serde_json;
use std::fs::File;
use std::io::prelude::*;
use std::{error::Error, path::PathBuf};

pub struct JinglesDb {
    db: Vec<Jingle>,
    path: PathBuf,
}

impl JinglesDb {
    pub fn new(json_file_path: PathBuf) -> Result<Self> {
        let db: Vec<Jingle> = Vec::new();
        let path = json_file_path;

        if let Err(_err) = File::open(&path) {
            let file = File::create(&path)?;
            serde_json::to_writer(file, &db)?;
        }

        Ok(Self { db, path })
    }

    pub fn load(&mut self) -> Result<()> {
        let mut file = File::open(&self.path)?;
        let mut buffer = String::new();
        file.read_to_string(&mut buffer)?;
        self.db = serde_json::from_str(buffer.as_str())?;
        Ok(())
    }

    pub fn save(&self) -> Result<()> {
        let f = File::open(&self.path)?;
        serde_json::to_writer(f, &self.db)?;
        Ok(())
    }

    pub fn get_random_jingle(&self) -> Jingle {
        let mut rng = rand::thread_rng();
        let index = rng.gen_range(0, self.db.len() - 1);
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
