use std::{error::Error, fs::File, io::BufReader};
use rodio::{Device, Sink};
use crate::jingles_db::{ JinglesDb, JingleDbObject };

pub struct JinglePlayer {
    stream: OutputStream,
    stream_handle: OutputStreamHandle,
    jingles_db: JinglesDb
}

impl JinglePlayer {
    pub fn new(jingles_path: String) -> Self {
        let jingles_db = JinglesDb::new(format!("{}/jingles/db.json", jingles_path)).unwrap();
        let device = rodio::default_output_device().unwrap();

        Self {
            device,
            jingles_db,
        }
    }

    pub fn play_file(&self, path: String) -> Result<(), PlayError>{
        let file = File::open(path).unwrap();
        let sink = rodio::play_once(&self.device, file)?;
        sink.sleep_until_end();
        println!("Finished playing file");
        Ok(())
    }

    pub fn play_random(&self) -> Result<(), PlayError>{
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