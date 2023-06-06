mod cli;
mod commands;

mod string_list;
pub use string_list::*;

use cli::Cli;
use manager::ProcessManager;

use crate::cli::Command;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    env_logger::init();
    let cli = Cli::init();
    match cli.command {
        Command::StartSwarm(cmd) => {
            let manifest = cli.common.load_manifest()?;
            let manager = ProcessManager::init(&cli.common.db_url, manifest).await?;
            cmd.run(manager).await
        }
        Command::CheckManifest(cmd) => cmd.run(cli.common.config_path.as_ref().unwrap()).await,
        Command::RunAction(cmd) => {
            let manifest = cli.common.load_manifest()?;
            let manager = ProcessManager::init(&cli.common.db_url, manifest).await?;
            cmd.run(manager).await
        }
    }
}
