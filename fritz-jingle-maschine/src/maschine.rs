mod jingle_player;

use jingle_player::JinglePlayer;
use sysfs_gpio::{Direction, Edge, Pin};
use std::path::PathBuf;
use eyre::Result;

pub struct Maschine {
    button: Pin,
    led: Option<Pin>,
    player: JinglePlayer
}

impl Maschine {
    pub fn new(jingles_path: PathBuf, button_pin: u64, led_pin: Option<u64>) -> Self {
        let player = JinglePlayer::new(jingles_path);
        let button = Pin::new(button_pin);
        let led: Option<Pin>;

        if let Some(pin) = led_pin {
            led = Some(Pin::new(pin));
        } else {
            led = None;
        }

        Self {
            button,
            led,
            player
        }
    }

    pub fn run(&mut self) -> Result<()>{
        self.button.with_exported(|| {
            self.button.set_direction(Direction::In)?;
            self.button.set_edge(Edge::FallingEdge)?;
            

            if let Some(led) = self.led {
                led.export()?;
                led.set_direction(Direction::Out)?;
                led.set_value(1)?;
            }

            let mut btn_poller = self.button.get_poller()?;
            let mut count = 0;
            println!("Ready");
            loop {
                let btn_value = btn_poller.poll(5)?;
                if let Some(val ) = btn_value {
                    
                    if val == 0 {
                        println!("Button press no {}", count);

                        if let Some(led) = self.led {
                            led.set_value(0)?;
                        }

                        if let Err(e) = self.player.play_random() {
                            dbg!(e);
                        };

                        if let Some(led) = self.led {
                            led.set_value(1)?;
                        }
                    }

                    count += 1;
                }
            }
        })?;

        Ok(())
    }
}