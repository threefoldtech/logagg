use yaml_rust::Yaml;
use anyhow::anyhow;

type DIR = String;

#[derive(Debug, Clone)]
pub enum Outputs {
    FILE(DIR),
}

impl Outputs {
}

impl TryFrom<&Yaml> for Outputs {
    type Error = anyhow::Error;

    fn try_from(value: &Yaml) -> Result<Self, Self::Error> {
        let typ = value["type"]
            .as_str()
            .ok_or(anyhow!("Can't parse type value"))?;

        match typ {
            "file" => {
                let dir = value["dir"]
                    .as_str()
                    .ok_or(anyhow!("Cannot parse dir value of type: {:?}", typ))?;
                    std::fs::create_dir_all(dir)?;
                Ok(Self::FILE(dir.to_string()))
            }
            _ => Err(anyhow!("Unsupported output type!")),
        }
    }
}