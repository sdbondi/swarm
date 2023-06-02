use std::str::FromStr;

pub enum ConfigFormat {
    Json,
    Toml,
    Yaml,
}

impl FromStr for ConfigFormat {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "json" => Ok(Self::Json),
            "toml" => Ok(Self::Toml),
            "yaml" => Ok(Self::Yaml),
            _ => Err(anyhow::anyhow!("Invalid config format")),
        }
    }
}
