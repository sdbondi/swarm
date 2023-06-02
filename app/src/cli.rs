//   Copyright 2023 The Tari Project
//   SPDX-License-Identifier: BSD-3-Clause

use crate::commands::check_config::CheckManifestCmd;
use crate::commands::spawn::StartSwarmCmd;
use clap::{Args, Parser, Subcommand};
use manifest::{load_from_file, SwarmManifest};
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
}

impl CommonArgs {
    pub fn load_manifest(&self) -> anyhow::Result<SwarmManifest> {
        // TODO: default config_path
        load_from_file(self.config_path.as_ref().unwrap())
    }
}
