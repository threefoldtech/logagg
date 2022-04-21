use anyhow::anyhow;
use anyhow::{Ok, Result};
use std::path::PathBuf;
use yaml_rust::YamlLoader;
use crate::output::Outputs;

#[derive(Default, Debug, Clone)]
pub struct Cfg {
    pub listen: String,
    pub outputs: Vec<Outputs>,
}

impl Cfg {
    pub fn new(cfg_path: PathBuf) -> Result<Self> {
        let yaml_str = std::fs::read_to_string(cfg_path)?;
        let yml = YamlLoader::load_from_str(&yaml_str)?;
        let listen = yml[0]["listen"]
            .as_str()
            .ok_or(anyhow!("Cannot parse Yaml listen address"))?
            .to_string();
        let yml_outputs = yml[0]["output"]
            .as_vec()
            .ok_or(anyhow!("Cannot parse Yaml output field"))?;
        let mut outputs = vec![];

        for op in yml_outputs {
            outputs.push(Outputs::try_from(op)?);
        }

        Ok(Self { listen, outputs })
    }
}
