use clap::{App, Arg};
use std::path::Path;
use super::maschine::Maschine;

pub struct Cli <'a> {
    app: App<'a>
}

impl<'a> Cli <'a> {
    pub fn new() -> Self {
        let app = App::new(env!("CARGO_PKG_NAME"))
        .version(env!("CARGO_PKG_VERSION"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .arg(
             Arg::new("FILES-PATH")
                 .about("Path to the Jingles files containing db.json and a folder called files containing the MP3s.")
                 .takes_value(true)
                 .short('f')
                 .long("files-path")
                 .required(true)
        )
        .arg(
             Arg::new("BUTTON-PIN")
             .about("Specifies the Raspberry Pi GPIO pin for the trigger button. BCM numbering is used.")
             .takes_value(true)
             .short('b')
             .long("button")
             .required(true)
        )
        .arg(
            Arg::new("LED-PIN")
            .about("Specifies the Raspberry Pi GPIO pin for the (optional) LED. BCM numbering is used.")
            .takes_value(true)
            .short('l')
            .long("led")
            .required(false)
        );

        Self {
            app
        }
    }

    pub fn process_arguments(&self) {
        // TODO: There has to be another solution to this than cloning?!
        let app = self.app.clone();
        let matches = app.get_matches().clone();
        
        let jingles_path;
        let button_pin;
        let led_pin: Option<u64>;

        if let Some(files_path) = matches.value_of("FILES-PATH") {
            jingles_path = Path::new(files_path).to_path_buf();
        } else {
            panic!();
        }

        if let Some(pin) = matches.value_of("BUTTON-PIN") {
            button_pin = pin.parse::<u64>().unwrap();
        } else {
            panic!();
        }

        if let Some(pin) = matches.value_of("LED-PIN") {
            led_pin = Some(pin.parse::<u64>().unwrap());
        } else {
            led_pin = None;
        }

        let mut jingle_maschine = Maschine::new(jingles_path, button_pin, led_pin);
        jingle_maschine.run().unwrap();
    }
}