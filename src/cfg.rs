use crate::output::Output;
use anyhow::{Ok, Result};
use std::path::PathBuf;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Cfg {
    pub listen: String,
    pub output: Vec<Output>,
}

impl Cfg {
    pub fn new(cfg_path: PathBuf) -> Result<Self> {
        let yaml_str = std::fs::read_to_string(cfg_path)?;
        Ok(serde_yaml::from_str(&yaml_str)?)
    }
}
