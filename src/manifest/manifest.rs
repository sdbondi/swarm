use serde::{Deserialize, Serialize};
use serde_with::serde_as;
use std::collections::HashMap;
use std::fmt::Display;
use std::ops::RangeInclusive;
use std::path::PathBuf;
use std::str::FromStr;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SwarmManifest {
    pub swarms: Vec<SwarmConfig>,
    #[serde(default)]
    pub actions: Vec<SwarmAction>,
    #[serde(default)]
    pub variables: HashMap<String, String>,
    #[serde(default)]
    pub instances: Vec<InstanceConfig>,
}

impl SwarmManifest {
    pub fn get_instance_group(&self, name: &str) -> &InstanceConfig {
        self.instances
            .iter()
            .find(|instance| instance.name == name)
            .expect("Instance group not found")
    }

    pub fn get_swarm(&self, name: &str) -> &SwarmConfig {
        self.swarms
            .iter()
            .find(|swarm| swarm.name == name)
            .expect("Swarm not found")
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SwarmConfig {
    pub name: String,
    pub executable: PathBuf,
    pub working_dir: Option<PathBuf>,
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
        params: serde_json::Value,
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InstanceIdRange {
    range: RangeInclusive<usize>,
}

impl InstanceIdRange {
    pub fn range(&self) -> RangeInclusive<usize> {
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
            end.parse::<usize>()? - 1
        };

        Ok(InstanceIdRange { range: start..=end })
    }
}

impl Display for InstanceIdRange {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}..={}", self.range.start(), self.range.end())
    }
}
