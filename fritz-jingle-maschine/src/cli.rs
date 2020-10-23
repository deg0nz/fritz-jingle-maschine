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
             .short('p')
             .long("pin")
             .required(true)
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

        let mut jingle_maschine = Maschine::new(button_pin, jingles_path);
        jingle_maschine.run().unwrap();
    }
}