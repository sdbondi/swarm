use crate::StringList;
use clap::Args;
use manager::ProcessManager;
use std::future::pending;

#[derive(Debug, Args, Clone)]
pub struct StartSwarmCmd {
    /// Instance group to spawn
    instances: StringList,
}

impl StartSwarmCmd {
    pub async fn run(&self, mut manager: ProcessManager) -> anyhow::Result<()> {
        for instance in &self.instances {
            manager.spawn_swarm(instance).await?;
        }

        // wait forever
        pending::<()>().await;

        Ok(())
    }
}
