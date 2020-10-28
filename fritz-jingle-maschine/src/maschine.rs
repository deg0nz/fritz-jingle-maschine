mod jingle_player;

use eyre::Result;
use jingle_player::JinglePlayer;
use std::{path::PathBuf, thread, time};
use sysfs_gpio::{Direction, Edge, Pin};

pub struct Maschine {
    button: Pin,
    led: Option<Pin>,
    player: JinglePlayer,
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
            player,
        }
    }

    fn signalize_ready_state(&self) -> Result<(), sysfs_gpio::Error> {
        println!("Ready");

        // Blink LED 3 times if connected
        if let Some(led) = self.led {
            let sleep_duration = time::Duration::from_millis(200);
            for _n in 1..3 {
                led.set_value(0)?;
                thread::sleep(sleep_duration);
                led.set_value(1)?;
                thread::sleep(sleep_duration);
            }
        }

        Ok(())
    }

    pub fn run(&mut self) -> Result<()> {
        self.button.with_exported(|| {
            self.button.set_direction(Direction::In)?;
            self.button.set_edge(Edge::FallingEdge)?;
            let mut btn_poller = self.button.get_poller()?;
            let mut count = 0;

            if let Some(led) = self.led {
                led.export()?;
                led.set_direction(Direction::Out)?;
                led.set_value(1)?;
            }

            self.signalize_ready_state()?;

            loop {
                let btn_value = btn_poller.poll(5)?;
                if let Some(val) = btn_value {
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
