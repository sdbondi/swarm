use crate::variables::Variables;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt::Display;
use std::ops::{RangeBounds, RangeInclusive};
use std::path::PathBuf;
use std::str::FromStr;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SwarmManifest {
    pub swarms: Vec<SwarmConfig>,
    #[serde(default)]
    pub actions: Vec<SwarmAction>,
    #[serde(default)]
    pub variables: Variables,
    #[serde(default)]
    pub instances: Vec<InstanceConfig>,
}

impl SwarmManifest {
    pub fn get_instance_group(&self, name: &str) -> Option<&InstanceConfig> {
        self.instances.iter().find(|instance| instance.name == name)
    }

    pub fn get_swarm(&self, name: &str) -> Option<&SwarmConfig> {
        self.swarms.iter().find(|swarm| swarm.name == name)
    }

    pub fn variables(&self) -> &Variables {
        &self.variables
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SwarmConfig {
    pub name: String,
    pub executable: String,
    pub working_dir: Option<String>,
    #[serde(default)]
    pub args: Vec<String>,
    #[serde(default)]
    pub env: HashMap<String, String>,
    #[serde(default)]
    pub ports: Vec<String>,
    #[serde(default)]
    pub on_first_start: Vec<String>,
    #[serde(default)]
    pub actions: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SwarmAction {
    pub name: String,
    #[serde(default)]
    pub restart_on_action: bool,
    #[serde(flatten)]
    pub action: Action,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Action {
    JsonRpc {
        url: String,
        method: String,
        params: Option<serde_json::Value>,
    },
    FsRm {
        path: PathBuf,
        force: bool,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct InstanceConfig {
    pub swarm: String,
    pub name: String,
    pub num_instances: Option<usize>,
    #[serde(with = "crate::serde::string::option")]
    pub id_range: Option<InstanceIdRange>,
}

impl InstanceConfig {
    pub fn get_id_range(&self) -> Option<InstanceIdRange> {
        self.id_range
            .clone()
            .or_else(|| Some((0..self.num_instances?).into()))
    }
}

pub type ProcessId = usize;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InstanceIdRange {
    range: RangeInclusive<ProcessId>,
}

impl InstanceIdRange {
    pub fn range(&self) -> RangeInclusive<ProcessId> {
        self.range.clone()
    }
}

impl FromStr for InstanceIdRange {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (start, end) = s
            .split_once("..")
            .ok_or_else(|| anyhow::anyhow!("Invalid node range"))?;
        if start.is_empty() || end.is_empty() {
            return Err(anyhow::anyhow!("start and end range cannot be empty"));
        }
        let start = start.parse()?;
        let end = if end.chars().nth(0).expect("empty checked") == '=' {
            end[1..].parse()?
        } else {
            end.parse::<ProcessId>()? - 1
        };

        Ok(InstanceIdRange { range: start..=end })
    }
}

impl Display for InstanceIdRange {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}..={}", self.range.start(), self.range.end())
    }
}

impl<T: RangeBounds<ProcessId>> From<T> for InstanceIdRange {
    fn from(value: T) -> Self {
        let start = match value.start_bound() {
            std::ops::Bound::Included(&start) => start,
            std::ops::Bound::Excluded(&start) => start + 1,
            std::ops::Bound::Unbounded => 0,
        };
        let end = match value.end_bound() {
            std::ops::Bound::Included(&end) => end,
            std::ops::Bound::Excluded(&end) => end - 1,
            std::ops::Bound::Unbounded => usize::MAX,
        };
        Self { range: start..=end }
    }
}
