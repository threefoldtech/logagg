use anyhow::{Ok, Result};
use serde::{Deserialize, Serialize};
use std::{fs::File, path::PathBuf};
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Output {
    pub kind: String,
    pub config: String,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Cfg {
    pub listen: String,
    pub output: Vec<Output>,
}

impl Cfg {
    pub fn new(cfg_path: PathBuf) -> Result<Self> {
        let file = File::open(cfg_path)?;
        let yml: Cfg = serde_yaml::from_reader(&file)?;
        Ok(yml)
    }
}
