use eyre::Result;
use fritz_jingle_db::{jingle::Jingle, JinglesDb};
use rodio::Device;
use std::{fs::File, path::PathBuf};

pub struct JinglePlayer {
    device: Device,
    jingles_db: JinglesDb,
    jingles_base_path: PathBuf,
}

impl JinglePlayer {
    pub fn new(jingles_base_path: PathBuf) -> Self {
        let device = rodio::default_output_device().unwrap();
        let db_path = jingles_base_path.join("db.json");
        let mut jingles_db = JinglesDb::new(db_path).unwrap();
        jingles_db.load().expect("Error loading Database");

        Self {
            device,
            jingles_db,
            jingles_base_path,
        }
    }

    pub fn play_file(&self, path: PathBuf) -> Result<()> {
        let file = File::open(path).unwrap();
        let sink = rodio::play_once(&self.device, file)?;
        sink.sleep_until_end();
        println!("Finished playing file");
        Ok(())
    }

    pub fn play_random(&self) -> Result<()> {
        let jingle = self.jingles_db.get_random_jingle();
        self.print_jingle_playing(&jingle);
        let jingle_file_path = self.jingles_base_path.join(jingle.file_path);
        println!("Jingle file path: {}", jingle_file_path.to_str().unwrap());
        self.play_file(jingle_file_path)?;
        Ok(())
    }

    fn print_jingle_playing(&self, jingle: &Jingle) {
        println!(
            "Playing {name} (Published on {date})",
            name = jingle.name,
            date = jingle.date_time
        );
    }
}
