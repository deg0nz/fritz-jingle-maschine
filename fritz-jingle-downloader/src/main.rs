mod cli;
mod downloader;

use cli::Cli;
use eyre::Error;
use futures::executor::block_on;

#[tokio::main]
async fn main() -> Result<(), Error> {
    // let mut rt = tokio::runtime::Runtime::new().unwrap();
    let cli = Cli::new();
    match block_on(cli.process_arguments()) {
        Ok(_) => println!("Done"),
        Err(e) => println!("An error ocurred: {}", e),
    };

    Ok(())
}
