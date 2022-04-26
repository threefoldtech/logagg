use crate::output::Output;
use anyhow::{Ok, Result};
use serde::{Deserialize, Serialize};
use std::{fs, path::PathBuf};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Cfg {
    pub listen: String,
    pub output: Vec<Output>,
}

impl Cfg {
    pub fn new(cfg_path: PathBuf) -> Result<Self> {
        let yaml_str = std::fs::read_to_string(cfg_path)?;
        let yml: Cfg = serde_yaml::from_str(&yaml_str)?;
        let outputs = yml.output.clone();

        for op in outputs {
            fs::create_dir_all(op.dir()).unwrap();
        }

        Ok(yml)
    }
}
