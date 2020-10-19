mod cli;
mod maschine;

use cli::Cli;

fn main() {
    let cli = Cli::new();
    cli.process_arguments();
}
