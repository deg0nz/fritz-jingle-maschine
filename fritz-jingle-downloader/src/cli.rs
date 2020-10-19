use std::path::Path;
use super::downloader::Downloader;
use eyre::Result;
use clap::{App, Arg, ArgMatches};
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
            Arg::new("FILES-PATH")
            .about("Downloads or updates all the jingles from Fritz to a given path. If a db.json is found in the path, an update is assumed.")
            .short('f')
            .long("files-path")
            .required(true)
        );

        Self {
            app
        }
    }

    pub async fn process_arguments(&self) {
        // TODO: There has to be another solution to this than cloning?!
        let app = self.app.clone();
        let matches = app.get_matches().clone();
         
        self.run_download(matches).await?;
    }

    async fn run_download(&self, matches: ArgMatches) {
        let jingles_path;

        if let Some(files_path) = matches.value_of("FILES-PATH") {
            jingles_path = Path::new(files_path).to_path_buf();
        } else {
            panic!();
        }
        
        let downloader = Downloader::new().await?;
        downloader.run();
    }
}