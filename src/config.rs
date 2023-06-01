//   Copyright 2023 The Tari Project
//   SPDX-License-Identifier: BSD-3-Clause

use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SwarmConfig {
    pub name: Option<String>,
    pub num_instances: usize,
    pub executable: PathBuf,
    pub args: Vec<String>,
    pub base_dir: PathBuf,
}
