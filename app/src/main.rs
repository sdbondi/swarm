#![feature(let_else)]

mod cli;
mod commands;
mod specific;

use cli::Cli;
use manager::ProcessManager;

use crate::cli::Command;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::init();
    match cli.command {
        Command::StartSwarm(cmd) => {
            let manifest = manifest::load_from_file(cli.common.config_path.as_ref().unwrap())?;
            let manager = ProcessManager::init(manifest)?;
            cmd.run(manager).await
        }
        Command::CheckManifest(cmd) => cmd.run(&cli.common.config_path).await,
    }
}
