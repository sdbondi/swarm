//   Copyright 2023 The Tari Project
//   SPDX-License-Identifier: BSD-3-Clause

use crate::config::SwarmConfig;
use crate::specific::do_specific_things;
use clap::{Args, Parser, Subcommand};
use std::fs::File;
use std::io::Read;
use std::path::{Path, PathBuf};

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
#[clap(propagate_version = true)]
pub struct Cli {
    #[clap(subcommand)]
    pub command: Command,
}

impl Cli {
    pub fn init() -> Self {
        Self::parse()
    }
}

#[allow(clippy::large_enum_variant)]
#[derive(Debug, Subcommand, Clone)]
pub enum Command {
    #[clap(alias = "start")]
    StartSwarm(StartSwarmCmd),
}

#[derive(Debug, Args, Clone)]
pub struct StartSwarmCmd {
    #[clap(short = 'c', long)]
    config_path: Option<PathBuf>,
    node_range: String,
}

impl StartSwarmCmd {
    pub async fn run(&self) {
        let node_range = parse_node_range(&self.node_range);
        do_specific_things(node_range).await.unwrap();
        // let config = load_config(&self.config_path);
        // let swarm = Swarm::spawn_from_config(config);
    }
}

fn parse_node_range(node_range: &str) -> std::ops::RangeInclusive<usize> {
    let mut parts = node_range.split("..");
    let start = parts.next().unwrap().parse().unwrap();
    let end = parts.next().unwrap().parse().unwrap();
    start..=end
}

fn load_config<P: AsRef<Path>>(path: P) -> anyhow::Result<SwarmConfig> {
    let mut config_file = File::open(&path).expect("Failed to open config file");

    match path.as_ref().extension().unwrap().to_str().unwrap() {
        "json" => {
            let config = serde_json::from_reader(config_file)?;
            Ok(config)
        }
        "toml" => {
            let mut s = String::new();
            config_file.read_to_string(&mut s)?;
            let config = toml::from_str(&s)?;
            Ok(config)
        }
        // "yaml" | "yml" => {
        //     yaml::parse_io(&mut config_file, ).expect("Failed to parse config file")
        // }
        s => panic!("Unsupported config file format '{}'", s),
    }
}
