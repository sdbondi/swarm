use clap::Args;
use colored::Colorize;
use manager::ProcessManager;

#[derive(Debug, Args, Clone)]
pub struct RunActionCmd {
    /// Instance group to use
    pub instance: String,
    /// Action to run
    pub action: String,
}

impl RunActionCmd {
    pub async fn run(&self, manager: ProcessManager) -> anyhow::Result<()> {
        let instances = manager
            .manifest()
            .get_instance_group(&self.instance)
            .ok_or(anyhow::anyhow!("Instance group not found"))?;

        let mut variables = manager.manifest().variables().clone();

        let mut action = manager
            .action_provider()
            .get_action(&self.action)
            .ok_or(anyhow::anyhow!("Action not found"))?;

        for instance in instances.get_id_range().unwrap().range() {
            print!(
                "Running action {} on instance: {}",
                self.action.yellow(),
                instance.to_string().yellow()
            );
            // TODO: This is very limiting/may not work as expected
            variables.set("swarm.instance.id", instance.to_string());
            action.execute(&variables).await?;
            println!("{}", " OK".green());
        }

        Ok(())
    }
}
