mod cli;
mod downloader;

use cli::Cli;
use futures::prelude::*;
use tokio::prelude::*;

#[tokio::main]
async fn main() {
    let mut rt = tokio::runtime::Runtime::new().unwrap();
    let cli = Cli::new();
    rt.block_on(cli.process_arguments());
}
