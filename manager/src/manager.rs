use crate::actions::ActionProvider;
use crate::models;
use crate::process::Process;
use crate::storage::ManagerStorage;
use anyhow::Context;
use manifest::SwarmManifest;
use sqlx::Acquire;

pub struct ProcessManager {
    storage: ManagerStorage,
    manifest: SwarmManifest,
    action_provider: ActionProvider,
    processes: Vec<Process>,
}

impl ProcessManager {
    pub async fn init(url: &str, manifest: SwarmManifest) -> anyhow::Result<Self> {
        let storage = ManagerStorage::init(url).await?;
        let action_provider = ActionProvider::new(manifest.actions.clone());
        Ok(Self {
            storage,
            manifest,
            action_provider,
            processes: Vec::new(),
        })
    }

    pub fn manifest(&self) -> &SwarmManifest {
        &self.manifest
    }

    pub fn action_provider(&self) -> &ActionProvider {
        &self.action_provider
    }

    pub async fn spawn_swarm(&mut self, name: &str) -> anyhow::Result<()> {
        let swarm = self
            .manifest
            .get_swarm(name)
            .ok_or(anyhow::anyhow!("Instance group not found"))?;

        let mut conn = self.storage.get_connection().await?;
        let mut tx = conn.begin().await?;
        for id in swarm.get_id_range().unwrap().range() {
            let is_first_start =
                !models::ProcessEntity::instance_exists(&mut tx, &swarm.name, id).await?;
            let process = Process::spawn(id, swarm, self, is_first_start)
                .await
                .context("Failed to spawn process")?;

            models::ProcessEntity::create_if_nexist(&mut tx, &swarm.name, &process).await?;
            println!("Spawned process: {}", process.instance_id());
            self.processes.push(process);
        }
        tx.commit().await?;

        Ok(())
    }
}
