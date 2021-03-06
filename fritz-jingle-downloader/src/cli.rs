use super::downloader::Downloader;
use clap::{App, Arg};
use eyre::Result;
use std::path::Path;

pub struct Cli<'a> {
    app: App<'a>,
}

impl<'a> Cli<'a> {
    pub fn new() -> Self {
        let app = App::new(env!("CARGO_PKG_NAME"))
        .version(env!("CARGO_PKG_VERSION"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .arg(
            Arg::new("JINGLES-PATH")
            .about("Downloads or updates all the jingles from Fritz to a given path. If a db.json is found in the path, missing jingles are downloaded.")
            .short('j')
            .long("jingles-path")
            .takes_value(true)
            .value_name("PATH")
            .required(true)
        );

        Self { app }
    }

    pub async fn process_arguments(&self) -> Result<()> {
        // TODO: There has to be another solution to this than cloning?!
        let app = self.app.clone();
        let matches = app.get_matches().clone();

        let jingles_path;

        if let Some(files_path) = matches.value_of("JINGLES-PATH") {
            jingles_path = Path::new(files_path).to_path_buf();
        } else {
            jingles_path = Path::new("./jingles").to_path_buf();
        }

        let mut downloader = Downloader::new(jingles_path).await?;
        downloader.run().await?;

        Ok(())
    }
}
