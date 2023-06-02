use manifest::{InstanceConfig, ProcessId, SwarmManifest};
use std::process::Stdio;
use tokio::{process, task};
use tokio_codec::{FramedRead, LinesCodec};
use tokio_stream::StreamExt;

pub struct Process {
    child: process::Child,
    id: ProcessId,
}

impl Process {
    pub fn spawn(
        id: ProcessId,
        instance: &InstanceConfig,
        manifest: &SwarmManifest,
    ) -> anyhow::Result<Self> {
        let mut vars = manifest.variables().clone();
        vars.set("id", id);

        let swarm = manifest
            .get_swarm(&instance.swarm)
            .ok_or(anyhow::anyhow!("Swarm '{}' not found", instance.swarm))?;

        let mut command = process::Command::new(&swarm.executable);

        if let Some(base_dir) = &swarm.working_dir {
            command.current_dir(base_dir.as_ref());
        }

        command
            .args(swarm.args.iter().map(|a| vars.substitute(a)))
            .kill_on_drop(true);

        let mut child = command.stdout(Stdio::piped()).spawn()?;
        let task_handle = task::spawn(async move {
            let mut stdout = child.stdout.take().unwrap();
            let mut stdout = FramedRead::new(stdout, LinesCodec::new());
            while let Some(line) = stdout.next().await {
                println!("{}: {}", id, line.unwrap());
            }
        });
    }
}
