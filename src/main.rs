pub mod jingles_db;
pub mod jingle_player;
pub mod cli;

use std::error::Error;

use sysfs_gpio::{Direction, Edge, Pin};
use jingle_player::JinglePlayer;
use cli::Cli;
use std::thread::sleep;
use std::time::Duration;

const BTN_PIN: u64 = 15;

fn main() {
    // let cli = Cli::new();
    // let jingles_path = cli.jingles_path;
    let jingles_path = "/home/pi/home/pi/Fritz-Jingle-Maschine/jingles".to_string();
    let player = JinglePlayer::new(jingles_path);
    let button = Pin::new(BTN_PIN);

    button.with_exported(|| {
        button.set_direction(Direction::In).unwrap();

        let mut btn_poller = button.get_poller().unwrap();
        let mut count = 0;

        loop {
            let btn_value = btn_poller.poll(5)?;
            if let Some(val ) = btn_value {
                if val == 0 {
                    println!("{}: Got 0", count);
                }
                // println!("{} Got button press on pin: {}", count, val);
                if let Err(e) = player.play_random() {
                    dbg!(e);
                };
                count += 1;
            }
            // sleep(Duration::from_millis(20));
        }
    }).unwrap();
}
