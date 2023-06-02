use crate::config_format::ConfigFormat;
use crate::manifest::SwarmManifest;
use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::str::FromStr;

pub fn load_from_file<P: AsRef<Path>>(path: P) -> anyhow::Result<SwarmManifest> {
    let config_fmt = path
        .as_ref()
        .extension()
        .unwrap()
        .to_str()
        .unwrap()
        .parse()?;
    let mut config_file = File::open(&path).expect("Failed to open config file");
    load_from_reader(config_file, config_fmt)
}

pub fn load_from_reader<R: Read>(
    mut reader: R,
    config_fmt: ConfigFormat,
) -> anyhow::Result<SwarmManifest> {
    match config_fmt {
        ConfigFormat::Json => {
            let config = serde_json::from_reader(reader)?;
            Ok(config)
        }
        ConfigFormat::Toml => {
            let mut s = String::new();
            reader.read_to_string(&mut s)?;
            let config = toml::from_str(&s)?;
            Ok(config)
        }
        ConfigFormat::Yaml => {
            todo!("YAML config format not yet implemented")
            //     yaml::parse_io(&mut reader, ).expect("Failed to parse config file")
        }
    }
}
