use crate::id_allocator::IdAllocator;
use crate::process::Process;
use crate::storage::ManagerStorage;
use manifest::{InstanceConfig, ProcessId, SwarmManifest};
use std::process;

pub struct ProcessManager {
    storage: ManagerStorage,
    manifest: SwarmManifest,
}

impl ProcessManager {
    pub async fn init(url: &str, manifest: SwarmManifest) -> anyhow::Result<Self> {
        let storage = ManagerStorage::init(url).await?;
        Ok(Self { storage, manifest })
    }

    pub fn spawn_instance(&self, instance: &str) -> anyhow::Result<()> {
        let instance = self
            .manifest
            .get_instance_group(instance)
            .ok_or(anyhow::anyhow!("Instance group not found"))?;

        for id in instance.get_id_range().unwrap().range() {
            let process = Process::spawn(id, instance, &self.manifest)?;
            // self.storage.add_instance(spawned)?;
        }

        Ok(())
    }
}
