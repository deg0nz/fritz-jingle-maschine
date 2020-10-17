use clap::{ App, Arg };
use std::{ffi::OsStr, path::{ PathBuf, Path }};

pub struct Cli {
    jingles_path: PathBuf,
    button_pin: u64
}

impl Cli {
    pub fn new() -> Self {
        let matches = App::new(env!("CARGO_PKG_NAME"))
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
             .about("Specifies the Raspberry Pi GPIO pin for the trigger button.")
             .takes_value(true)
             .short('p')
             .long("pin")
             .required(true)
            )
        .get_matches(); 
 
        let jingles_path;
        if let Some(files_path) = matches.value_of("FILES-PATH") {
            jingles_path = Path::new(files_path).to_path_buf();
        } else {
            panic!();
        }

        let button_pin;
        if let Some(pin) = matches.value_of("BUTTON-PIN") {
            button_pin = pin.parse::<u64>().unwrap();
        } else {
            panic!();
        }

        Self {
            jingles_path,
            button_pin
        }
    }

    pub fn get_jingles_path(&self) -> PathBuf {
        self.jingles_path.clone()
    }

    pub fn get_button_pin(&self) -> u64 {
        self.button_pin.clone()
    }
}