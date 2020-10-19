mod cli;
mod maschine;
mod downloader;

use cli::Cli;

fn main() {
    let cli = Cli::new();
    cli.process_arguments();
}
