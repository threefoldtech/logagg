use super::{Output, OutputDriver};
use anyhow::{Context, Result};
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

pub struct FileDriver {
    dir: PathBuf,
}

impl FileDriver {
    pub fn new<S: Into<PathBuf>>(dir: S) -> Result<FileDriver> {
        let dir = dir.into();
        std::fs::create_dir_all(&dir).context("failed to create output directory")?;

        Ok(FileDriver { dir })
    }
}

struct DirOutput(String, File);

impl Output for DirOutput {
    fn write(&mut self, b: &[u8]) -> Result<()> {
        let _ = self.1.write(b)?;
        Ok(())
    }

    fn id(&self) -> &str {
        &self.0
    }
}

#[async_trait::async_trait]
impl OutputDriver for FileDriver {
    fn id(&self) -> &str {
        "file-driver"
    }

    async fn open(&self, name: &str) -> Result<Box<dyn Output>> {
        use tokio::fs::OpenOptions;

        let path = self.dir.clone().join(name);
        let file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&path)
            .await?;

        let id = match path.to_str() {
            None => "unknown",
            Some(id) => id,
        };

        Ok(Box::new(DirOutput(id.into(), file.into_std().await)))
    }
}
