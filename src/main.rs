mod cli;
mod config;
mod node;
mod specific;
mod swarm;

use cli::Cli;

use crate::cli::Command;

#[tokio::main]
async fn main() {
    let cli = Cli::init();
    match cli.command {
        Command::StartSwarm(start) => start.run().await,
    }
}
