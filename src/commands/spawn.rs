use crate::cli::CommonArgs;
use crate::manifest;
use clap::Args;

#[derive(Debug, Args, Clone)]
pub struct StartSwarmCmd {}

impl StartSwarmCmd {
    pub async fn run(&self, common: &CommonArgs) -> anyhow::Result<()> {
        let manifest = manifest::load_from_file(common.config_path.as_ref().unwrap())?;
        let instances = manifest.get_instance_group("swarm1");
        crate::specific::do_specific_things(instances.spawn_range.clone().unwrap().range())
            .await
            .unwrap();

        Ok(())
    }
}
