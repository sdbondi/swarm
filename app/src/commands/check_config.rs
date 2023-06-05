use crate::cli::CommonArgs;
use clap::Args;
use std::path::Path;

#[derive(Debug, Args, Clone)]
pub struct CheckManifestCmd {}

impl CheckManifestCmd {
    pub async fn run<P: AsRef<Path>>(&self, path: P) -> anyhow::Result<()> {
        let config = manifest::load_from_file(path)?;
        eprintln!("{}", serde_json::to_string_pretty(&config).unwrap());
        Ok(())
    }
}
