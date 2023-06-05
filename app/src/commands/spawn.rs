use crate::cli::CommonArgs;
use clap::Args;
use manager::ProcessManager;
use tokio::time;

#[derive(Debug, Args, Clone)]
pub struct StartSwarmCmd {
    /// Instance group to spawn
    instance: String,
}

impl StartSwarmCmd {
    pub async fn run(&self, mut manager: ProcessManager) -> anyhow::Result<()> {
        manager.spawn_instance_group(&self.instance).await?;
        time::sleep(time::Duration::from_secs(100)).await;

        Ok(())
    }
}
