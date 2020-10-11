pub mod jingles_db;
pub mod jingle_player;
pub mod cli;

use std::error::Error;

use sysfs_gpio::{ 
    Direction, Pin };
use jingle_player::JinglePlayer;
use cli::Cli;

const BTN_PIN: u64 = 26;

fn main() -> Result<(), Box<dyn Error>> {
    let cli = Cli::new();
    let player = JinglePlayer::new(cli.jingles_path);
    let button = Pin::new(BTN_PIN);
    button.set_direction(Direction::In)?;

    let mut btn_poller = button.get_poller()?;

    loop {
        let btn_value = btn_poller.poll(10)?;
        if let Some(val ) = btn_value {
            println!("Got button press");
            if let Err(e) = player.play_random() {
                dbg!(e);
            } ;
        }
    }
}
