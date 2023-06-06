use crate::actions::SwarmAction;
use anyhow::Context;
use async_trait::async_trait;
use log::*;
use manifest::Variables;
use std::path::PathBuf;
use tokio::fs;

pub struct FsRmAction {
    pub path: PathBuf,
    pub force: bool,
}

#[async_trait]
impl SwarmAction for FsRmAction {
    async fn execute(&mut self, vars: &Variables) -> anyhow::Result<()> {
        let path = vars.substitute(
            self.path
                .as_os_str()
                .to_str()
                .ok_or_else(|| anyhow::anyhow!("Invalid path"))?,
        );
        info!("Removing {}", path);
        if self.force {
            // force implies we dont care about errors e.g dir doesnt exist
            let _ = fs::remove_dir_all(path).await;
        } else {
            fs::remove_dir(path)
                .await
                .context("remove_dir in FsRmAction")?;
        }
        Ok(())
    }
}
