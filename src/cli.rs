use clap::{ App, Arg };

pub struct Cli {
    pub jingles_path: String
}

impl Cli {
    pub fn new() -> Self{
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
        .get_matches(); 
 
        let jingles_path;

        if let Some(files_path) = matches.value_of("FILES-PATH") {
            jingles_path = files_path.to_string();
        } else {
            panic!()
        }

        Self {
            jingles_path
        }
    }
}