mod jingles_db;
mod jingle_player;

use jingle_player::JinglePlayer;
use sysfs_gpio::{Direction, Pin};
use std::path::PathBuf;
use eyre::Result;

pub struct Maschine {
    button: Pin,
    player: JinglePlayer
}

impl Maschine {
    pub fn new(button_pin: u64, jingles_path: PathBuf) -> Self{
        let player = JinglePlayer::new(jingles_path);
        let button = Pin::new(button_pin);

        Self {
            button,
            player
        }
    }

    pub fn run(&mut self) -> Result<()>{
        self.button.with_exported(|| {
            self.button.set_direction(Direction::In).unwrap();
    
            let mut btn_poller = self.button.get_poller().unwrap();
            let mut count = 0;
    
            loop {
                let btn_value = btn_poller.poll(5)?;
                if let Some(val ) = btn_value {
                    if val == 0 {
                        println!("{}: Got 0", count);
                    }
                    // println!("{} Got button press on pin: {}", count, val);
                    if let Err(e) = self.player.play_random() {
                        dbg!(e);
                    };
                    count += 1;
                }
                // sleep(Duration::from_millis(20));
            }
        })?;

        Ok(())
    }
}