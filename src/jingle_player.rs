use std::{error::Error, fs::File, io::BufReader};
use rodio::{Device, Source};

use crate::jingles_db::JinglesDb;

pub struct JinglePlayer {
    device: Device,
    jingles_db: JinglesDb,
    jingles_path: String
}

impl JinglePlayer {
    pub fn new(jingles_path: String) -> Self {
        let device = rodio::default_output_device().unwrap();
        let jingles_db = JinglesDb::new(format!("{}/db.json", jingles_path)).unwrap();

        Self {
            device,
            jingles_db,
            jingles_path
        }
    }

    fn play_file(&self, path: String) -> Result<(), Box<dyn Error>>{
        let file = File::open(path).unwrap();
        // let source = rodio::Decoder::new(BufReader::new(file)).unwrap();
        let sink = rodio::play_once(&self.device, file)?;
        sink.sleep_until_end();
        Ok(())
    }

    pub fn play_random(&self) {
        let jingle = self.jingles_db.get_random_entry();
        let file_path = format!("{}{}", self.jingles_path, jingle.file_path);
        self.play_file(file_path).unwrap();
    }
}