//   Copyright 2023 The Tari Project
//   SPDX-License-Identifier: BSD-3-Clause

use crate::manifest::{SwarmConfig, SwarmManifest};
use crate::node::Node;
use tokio::process::Command;

pub struct Swarm {
    nodes: Vec<Node>,
}

impl Swarm {
    pub fn new() -> Self {
        Self { nodes: Vec::new() }
    }

    pub fn spawn_from_manifest(manifest: SwarmManifest) -> anyhow::Result<Self> {
        todo!()
        // let instances = (0..config.num_instances)
        //     .map(|_| Command::new(config.executable).args(config.args).spawn())
        //     .collect::<Result<Vec<_>, _>>()?;
        //
        // Ok(Self {
        //     nodes: instances
        //         .into_iter()
        //         .map(|ch| Node {
        //             name: config.name.clone().unwrap_or_else(|| "UNNAMED".to_string()),
        //             process: ch,
        //         })
        //         .collect(),
        // })
    }
}
