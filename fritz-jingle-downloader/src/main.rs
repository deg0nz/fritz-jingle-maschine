mod cli;
mod downloader;

use cli::Cli;

// TODO: use tokio macro to automatically setup runtine as in https://github.com/benkay86/async-applied/blob/master/reqwest-tokio/src/main.rs
fn main() {
    let mut rt = tokio::runtime::Runtime::new().unwrap();
    let cli = Cli::new();
    match rt.block_on(cli.process_arguments()) {
        Ok(_) => println!("Done"),
        Err(e) => println!("An error ocurred: {}", e),
    };
}
