//   Copyright 2023 The Tari Project
//   SPDX-License-Identifier: BSD-3-Clause

use crate::commands::check_config::CheckManifestCmd;
use crate::commands::spawn::StartSwarmCmd;
use clap::{Args, Parser, Subcommand};
use manifest::{load_from_file, SwarmManifest};
use std::env;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
#[clap(propagate_version = true)]
pub struct Cli {
    #[clap(flatten)]
    pub common: CommonArgs,
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
    #[clap(alias = "check")]
    CheckManifest(CheckManifestCmd),
}

#[derive(Debug, Args, Clone)]
pub struct CommonArgs {
    #[clap(short = 'c', long)]
    pub config_path: Option<PathBuf>,
    #[clap(
        short = 'd',
        long,
        default_value = "sqlite:./swarm.sqlite",
        env = "SWARM_DATABASE"
    )]
    pub db_url: String,
}

impl CommonArgs {
    pub fn load_manifest(&self) -> anyhow::Result<SwarmManifest> {
        let config = self.config_path.as_ref().unwrap();
        let config = if config.is_relative() {
            env::current_dir()?.join(config)
        } else {
            config.clone()
        };
        // TODO: default config_path
        load_from_file(config)
    }
}
