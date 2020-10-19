mod cli;
mod downloader;

use cli::Cli;

fn main() {
    let mut rt = tokio::runtime::Runtime::new().unwrap();
    let cli = Cli::new();
    match rt.block_on(cli.process_arguments()) {
        Ok(_) => println!("Done"),
        Err(e) => println!("An error ocurred: {}", e),
    };
}
