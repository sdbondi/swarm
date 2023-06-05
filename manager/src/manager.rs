use crate::models;
use crate::process::Process;
use crate::storage::ManagerStorage;
use anyhow::Context;
use manifest::SwarmManifest;
use sqlx::{Acquire, Connection};

pub struct ProcessManager {
    storage: ManagerStorage,
    manifest: SwarmManifest,

    processes: Vec<Process>,
}

impl ProcessManager {
    pub async fn init(url: &str, manifest: SwarmManifest) -> anyhow::Result<Self> {
        let storage = ManagerStorage::init(url).await?;
        Ok(Self {
            storage,
            manifest,
            processes: Vec::new(),
        })
    }

    pub async fn spawn_instance_group(&mut self, instance: &str) -> anyhow::Result<()> {
        let instance = self
            .manifest
            .get_instance_group(instance)
            .ok_or(anyhow::anyhow!("Instance group not found"))?;

        let mut access = self.storage.write_access().await?;
        let mut tx = access.connection.begin().await?;
        for id in instance.get_id_range().unwrap().range() {
            let process = Process::spawn(id, instance, &self.manifest)
                .await
                .context("Failed to spawn process")?;
            let entity = models::ProcessEntity::create(&mut tx, &instance.name, &process).await?;
            println!("Spawned process: {:?}", entity);
            self.processes.push(process);
        }
        tx.commit().await?;

        Ok(())
    }
}
