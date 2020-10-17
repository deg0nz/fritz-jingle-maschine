use std::{error::Error, fs::File, io::BufReader, path::PathBuf};
use rodio::{Device, Sink};
use crate::jingles_db::{ JinglesDb, JingleDbObject };
use eyre::Result;

pub struct JinglePlayer {
    device: Device,
    jingles_db: JinglesDb,
    jingles_base_path: PathBuf
}

impl JinglePlayer {
    pub fn new(mut jingles_base_path: PathBuf) -> Self {
        let device = rodio::default_output_device().unwrap();
        let db_path = jingles_base_path.join("db.json");
        let jingles_db = JinglesDb::new(db_path).unwrap();

        Self {
            device,
            jingles_db,
            jingles_base_path
        }
    }

    pub fn play_file(&self, path: PathBuf) -> Result<()>{
        let file = File::open(path).unwrap();
        let sink = rodio::play_once(&self.device, file)?;
        sink.sleep_until_end();
        println!("Finished playing file");
        Ok(())
    }

    pub fn play_random(&self) -> Result<()>{
        let jingle = self.jingles_db.get_random_entry();
        self.print_jingle_playing(&jingle);
        let jingle_file_path = self.jingles_base_path.join(jingle.file_path);
        println!("Jingle file path: {}", jingle_file_path.to_str().unwrap());
        self.play_file(jingle_file_path)?;
        Ok(())
    }

    fn print_jingle_playing(&self, jingle: &JingleDbObject) {
        println!("Playing {name} (Published on {date})", name = jingle.name, date = jingle.date_time);
    }
}