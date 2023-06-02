mod cli;
mod commands;
mod manifest;
mod node;
mod serde;
mod specific;
mod swarm;

use cli::Cli;

use crate::cli::Command;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::init();
    match cli.command {
        Command::StartSwarm(cmd) => cmd.run(&cli.common).await,
        Command::CheckConfig(cmd) => cmd.run(&cli.common).await,
    }
}
