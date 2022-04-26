use std::{fs::OpenOptions, io::Write};

use anyhow::{Ok, Result};
use serde::{Deserialize, Serialize};

type DIR = String;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", content = "dir")]
pub enum Output {
    File(DIR),
    Xyz(DIR),
}

impl Output {
    pub fn dir(&self) -> DIR {
        match self {
            Output::File(dir) => dir.to_owned(),
            Output::Xyz(dir) => dir.to_owned(),
        }
    }

    pub fn write(&self, filename: &String, text: &[u8]) -> Result<()> {
        
        match self {
            Output::File(dir) => {
                let filepath = format!("{}/{}", dir.trim_end_matches("/"), filename);
                let mut file = OpenOptions::new()
                    .create(true)
                    .append(true)
                    .open(filepath)?;

                file.write_all(text)?;
            }
            Output::Xyz(_dir) => {
                // unimplemented!()
            }
        }

        Ok(())
    }
}
