pub mod jingle;

use eyre::Result;
use jingle::Jingle;
use rand::Rng;
use serde_json;
use std::fs::{File, OpenOptions};
use std::io::prelude::*;
use std::path::PathBuf;

#[derive(Debug)]
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
        let f = OpenOptions::new().write(true).open(&self.path)?;
        serde_json::to_writer_pretty(f, &self.db)?;
        Ok(())
    }

    // TODO: This should return a Result/Option because the DB could be empty!
    pub fn get_random_jingle(&self) -> Jingle {
        let mut rng = rand::thread_rng();
        let index = rng.gen_range(0, self.db.len() - 1);
        let db_object = self.db[index].clone();
        println!("Selected jingle no {} with name: {}", index, db_object.name);
        db_object
    }

    pub fn add_jingle(&mut self, jingle: Jingle) {
        if !self.db.contains(&jingle) {
            self.db.push(jingle);
        }
    }

    pub fn contains(&self, jingle: &Jingle) -> bool {
        let contains = self.db.contains(jingle);
        contains
    }

    pub fn push_list(&mut self, list: &mut Vec<Jingle>) {
        for jingle in list.iter() {
            self.add_jingle(jingle.to_owned())
        }
    }

    pub fn is_empty(&self) -> bool {
        self.db.is_empty()
    }

    pub fn set_db(&mut self, jingles: Vec<Jingle>) {
        self.db = jingles
    }
}
