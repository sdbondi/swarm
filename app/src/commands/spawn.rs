use crate::cli::CommonArgs;
use clap::Args;
use manager::ProcessManager;

#[derive(Debug, Args, Clone)]
pub struct StartSwarmCmd {
    /// Instance group to spawn
    instance: String,
}

impl StartSwarmCmd {
    pub async fn run(&self, manager: ProcessManager) -> anyhow::Result<()> {
        crate::specific::do_specific_things(instances.id_range.clone().unwrap().range())
            .await
            .unwrap();

        Ok(())
    }
}
