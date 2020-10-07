pub mod jingles_db;
pub mod jingle_player;
pub mod cli;

use sysfs_gpio::{ Pin, PinPoller, Direction };
use jingle_player::JinglePlayer;
use cli::Cli;

const BTN_PIN: u64 = 5;

fn main() {
    let cli = Cli::new();
    let player = JinglePlayer::new(cli.jingles_path);
    let button = Pin::new(BTN_PIN);
    button.set_direction(Direction::In).unwrap();

    let mut btn_poller = button.get_poller().unwrap();

    loop {
        let btn_value = btn_poller.poll(10).unwrap();
        if let Some(val ) = btn_value {
            println!("Got button press");
            player.play_random();
        }
    }
}
