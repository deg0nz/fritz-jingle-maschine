use clap::{App, Arg, ArgMatches};
use std::path::Path;
use super::maschine::Maschine;
use super::downloader::Downloader;
use eyre::Result;
use futures::poll;

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
            Arg::new("RUN")
                .takes_value(false)
                .about("Runs the main app")
                .short('r')
                .long("run")
                .requires_all(&["FILES-PATH", "BUTTON-PIN"])
        )
        .arg(
             Arg::new("FILES-PATH")
                 .about("Path to the Jingles files containing db.json and a folder called files containing the MP3s.")
                 .takes_value(true)
                 .short('f')
                 .long("files-path")
                 .default_value(".")
        )
        .arg(
             Arg::new("BUTTON-PIN")
             .about("Specifies the Raspberry Pi GPIO pin for the trigger button. BCM numbering is used.")
             .takes_value(true)
             .short('p')
             .long("pin")
             .required_unless_present("DOWNLOAD")
            )
        .arg(
            Arg::new("DOWNLOAD")
            .about("Downloads or updates all the jingles from Fritz to a given path. If a db.json is found in the path, an update is assumed.")
            .short('d')
            .long("download-jingles")
            .required_unless_present_any(&["RUN", "BUTTON-PIN"])
            .requires("FILES-PATH")
        );

        Self {
            app
        }
    }

    pub fn process_arguments(&self) {
        // TODO: There has to be another solution to this than cloning?!
        let app = self.app.clone();
        let matches = app.get_matches().clone();
         
        if matches.is_present("RUN") {
            self.run_maschine(matches);
        } else if matches.is_present("DOWNLOAD") {
            self.run_download(matches);
        }
    }

    fn run_maschine(&self, matches: ArgMatches) {
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

        let mut jingles_maschine = Maschine::new(button_pin, jingles_path);
        jingles_maschine.run().unwrap();
    }

    fn run_download(&self, matches: ArgMatches) {
        let jingles_path;

        if let Some(files_path) = matches.value_of("FILES-PATH") {
            jingles_path = Path::new(files_path).to_path_buf();
        } else {
            panic!();
        }

        
        //  block_on(Downloader::new().await);
        //     downloader.run();
    }
}