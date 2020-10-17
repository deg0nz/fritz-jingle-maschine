use std::{error::Error, fs::File, io::BufReader};
use rodio::{OutputStream, OutputStreamHandle, PlayError};
use crate::jingles_db::{ JinglesDb, JingleDbObject };

pub struct JinglePlayer {
    stream: OutputStream,
    stream_handle: OutputStreamHandle,
    jingles_db: JinglesDb
}

impl JinglePlayer {
    pub fn new(jingles_path: String) -> Self {
        let (stream, stream_handle) = rodio::OutputStream::try_default().unwrap();
        let jingles_db = JinglesDb::new(format!("{}/jingles/db.json", jingles_path)).unwrap();

        Self {
            stream,
            stream_handle,
            jingles_db
        }
    }

    pub fn play_file(&self, path: String) -> Result<(), PlayError>{
        let file = File::open(path).unwrap();
        let sink = self.stream_handle.play_once(BufReader::new(file))?;
        sink.sleep_until_end();
        println!("Finished playing file");
        Ok(())
    }

    pub fn play_random(&self) -> Result<(), PlayError>{
        let jingle = self.jingles_db.get_random_entry();
        self.print_jingle_playing(&jingle);
        self.play_file(jingle.file_path)?;
        Ok(())
    }

    fn print_jingle_playing(&self, jingle: &JingleDbObject) {
        println!("Playing {name} (Published on {date})", name = jingle.name, date = jingle.date_time);
    }
}