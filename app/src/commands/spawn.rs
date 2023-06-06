use clap::Args;
use manager::ProcessManager;
use std::future::pending;

#[derive(Debug, Args, Clone)]
pub struct StartSwarmCmd {
    /// Instance group to spawn
    instance: String,
}

impl StartSwarmCmd {
    pub async fn run(&self, mut manager: ProcessManager) -> anyhow::Result<()> {
        manager.spawn_instance_group(&self.instance).await?;

        // wait forever
        pending::<()>().await;

        Ok(())
    }
}
