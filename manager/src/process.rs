use crate::actions::ActionProvider;
use crate::ProcessManager;
use anyhow::{anyhow, Context};
use colored::Colorize;
use log::*;
use manifest::{InstanceConfig, InstanceId, SwarmConfig, SwarmManifest, Variables};
use std::collections::{HashMap, VecDeque};
use std::net::IpAddr;
use std::process::Stdio;
use std::time::Duration;
use tokio::io::{AsyncBufReadExt, BufReader};
use tokio::net::TcpListener;
use tokio::sync::{mpsc, oneshot};
use tokio::time::Instant;
use tokio::{process, task, time};

pub struct Process {
    // child: process::Child,
    instance_id: InstanceId,
    pid: u32,
    sender: mpsc::Sender<ProcessCommand>,
}

impl Process {
    pub fn instance_id(&self) -> InstanceId {
        self.instance_id
    }

    pub fn pid(&self) -> u32 {
        self.pid
    }

    pub async fn spawn(
        instance_id: InstanceId,
        instance: &InstanceConfig,
        manager: &ProcessManager,
        is_first_start: bool,
    ) -> anyhow::Result<Self> {
        let mut vars = manager.manifest().variables().clone();
        vars.set("id", instance_id);

        let swarm = manager
            .manifest()
            .get_swarm(&instance.swarm)
            .ok_or_else(|| anyhow::anyhow!("Swarm '{}' not found", instance.swarm))?;

        let allocated_ports = allocate_ports(&swarm.ports).await?;
        for (name, port) in allocated_ports {
            vars.set(format!("ports[{}]", name), port);
            vars.set(format!("swarm.instance.ports[{}]", name), port);
            vars.set("swarm.instance.id", instance_id);
        }

        let exec = vars.substitute(&swarm.executable);
        info!("Spawning process {} with executable {}", instance_id, exec);
        let mut command = process::Command::new(exec);

        if let Some(ref base_dir) = swarm.working_dir {
            let base_dir = vars.substitute(base_dir);
            tokio::fs::create_dir_all(&base_dir)
                .await
                .context("Failed to create working directory")?;
            info!("working directory is {}", base_dir);
            command.current_dir(base_dir);
        }

        info!(
            "Spawning child process: {}",
            swarm
                .args
                .iter()
                .map(|a| vars.substitute(a))
                .collect::<Vec<String>>()
                .join(" ")
        );

        let child = command
            .envs(swarm.env.iter().map(|(k, v)| (k, vars.substitute(v))))
            .args(swarm.args.iter().map(|a| vars.substitute(a)))
            .kill_on_drop(true)
            .stdout(Stdio::piped())
            .spawn()
            .context("Failed to spawn child process")?;
        let pid = child.id().ok_or_else(|| {
            anyhow!(
                "Instance {}{} has exited unexpectedly",
                instance.name,
                instance_id
            )
        })?;

        let action_provider = manager.action_provider().clone();
        let sender = ProcessWorker::spawn(
            instance_id,
            child,
            swarm.clone(),
            manager.manifest().clone(),
            action_provider,
            vars,
            is_first_start,
        );

        Ok(Self {
            instance_id,
            pid,
            sender,
        })
    }
}

struct ProcessWorker {
    id: InstanceId,
    child: process::Child,
    receiver: mpsc::Receiver<ProcessCommand>,
    lines_buf: VecDeque<String>,
    config: SwarmConfig,
    manifest: SwarmManifest,
    action_provider: ActionProvider,
    vars: Variables,
    is_first_start: bool,
}

impl ProcessWorker {
    pub fn spawn(
        instance_id: InstanceId,
        child: process::Child,
        config: SwarmConfig,
        manifest: SwarmManifest,
        action_provider: ActionProvider,
        vars: Variables,
        is_first_start: bool,
    ) -> mpsc::Sender<ProcessCommand> {
        let (tx, rx) = mpsc::channel(1);
        let worker = Self {
            id: instance_id,
            child,
            receiver: rx,
            lines_buf: VecDeque::with_capacity(1000),
            config,
            manifest,
            action_provider,
            vars,
            is_first_start,
        };

        task::spawn(worker.run());
        tx
    }

    pub async fn run(mut self) {
        let stdout = self.child.stdout.take().unwrap();
        let mut stdout = BufReader::new(stdout).lines();
        let after_start_timer = time::sleep(Duration::from_secs(5));
        tokio::pin!(after_start_timer);

        loop {
            tokio::select! {
                Some(req) = self.receiver.recv() => {
                     match req {
                        ProcessCommand::Kill(reply) => {
                            let _ = reply.send(self.child.kill().await.context("Kill process"));
                            break;
                        }
                        ProcessCommand::Status(reply) => {
                            let _ = reply.send(Ok(self.child.try_wait().is_err()));
                        }
                        ProcessCommand::GetLines { from, to, reply } => {
                            let lines = self.lines_buf.iter().skip(from).take(to - from).cloned().collect::<Vec<_>>();
                            let _ = reply.send(Ok(lines));
                        }
                    }
                },

                Ok(Some(line)) = stdout.next_line() => {
                    println!("[{}|{}] {}", self.config.name.red(), self.id.to_string().green(), line);

                    assert!(self.lines_buf.len() <= 1000);
                    if self.lines_buf.len() == 1000 {
                        self.lines_buf.pop_front();
                    }
                    self.lines_buf.push_back(line);
                },

                _ = &mut after_start_timer => {
                    // Never again
                    after_start_timer.as_mut().reset(Instant::now() + Duration::from_secs(10000*365*24*60*60));
                    // TODO: check that all allocated ports are active / pidfile exists before considering ready
                    if self.is_first_start {
                        info!("[{}] AFTER FIRST START triggered", self.id);
                        if let Err(err) = self.on_after_first_start().await {
                            error!("Failed to run on_first_start: {}", err);
                        }
                    }

                    info!("[{}] AFTER START triggered", self.id);
                    if let Err(err) = self.on_after_start().await {
                        error!("Failed to run on_start: {}", err);
                    }
                }
            }
        }
    }

    async fn on_after_start(&self) -> anyhow::Result<()> {
        for name in &self.config.actions.on_after_start {
            self.execute_action(name).await?;
        }
        Ok(())
    }

    async fn on_after_first_start(&self) -> anyhow::Result<()> {
        for name in &self.config.actions.on_after_first_start {
            self.execute_action(name).await?;
        }
        Ok(())
    }

    async fn execute_action(&self, name: &str) -> anyhow::Result<()> {
        let mut action = self
            .action_provider
            .get_action(name)
            .ok_or_else(|| anyhow::anyhow!("Action '{}' not found for on_first_start", name))?;
        action.execute(&self.vars).await
    }
}

type Reply<T> = oneshot::Sender<anyhow::Result<T>>;

pub enum ProcessCommand {
    Kill(Reply<()>),
    Status(Reply<bool>),
    GetLines {
        from: usize,
        to: usize,
        reply: Reply<Vec<String>>,
    },
}

async fn allocate_ports(ports: &[String]) -> anyhow::Result<HashMap<&str, u16>> {
    let mut allocated_ports = HashMap::with_capacity(ports.len());
    // TODO: Lots of potential problems here
    for name in ports {
        let listener = TcpListener::bind((IpAddr::from([127u8, 0, 0, 1]), 0u16))
            .await
            .context("[allocated_ports] OS-assigned port failed")?;
        allocated_ports.insert(name.as_str(), listener.local_addr()?.port());
    }
    Ok(allocated_ports)
}
