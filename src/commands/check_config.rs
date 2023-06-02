use crate::cli::CommonArgs;
use crate::manifest;
use clap::Args;

#[derive(Debug, Args, Clone)]
pub struct CheckConfigCmd {}

impl CheckConfigCmd {
    pub async fn run(&self, common: &CommonArgs) -> anyhow::Result<()> {
        let config = manifest::load_from_file(common.config_path.as_ref().unwrap())?;
        eprintln!("{}", serde_json::to_string_pretty(&config).unwrap());
        Ok(())
    }
}
