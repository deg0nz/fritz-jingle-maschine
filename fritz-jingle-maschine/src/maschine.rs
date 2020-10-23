mod jingle_player;

use jingle_player::JinglePlayer;
use sysfs_gpio::{Direction, Edge, Pin};
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
            self.button.set_direction(Direction::In)?;
            self.button.set_edge(Edge::FallingEdge)?;
            let mut btn_poller = self.button.get_poller()?;
            let mut count = 0;
            println!("Ready");
            loop {
                let btn_value = btn_poller.poll(5)?;
                if let Some(val ) = btn_value {
                    
                    if val == 0 {
                        println!("{}: Got 0", count);

                        if let Err(e) = self.player.play_random() {
                            dbg!(e);
                        };
                    }

                    count += 1;
                }
            }
        })?;

        Ok(())
    }
}