use anyhow::Result;
use async_trait::async_trait;

mod file;

pub trait Output {
    fn id(&self) -> &str;
    fn write(&mut self, b: &[u8]) -> Result<()>;
}

#[async_trait]
pub trait OutputDriver: Sync + Send + 'static {
    fn id(&self) -> &str;
    async fn open(&self, name: &str) -> Result<Box<dyn Output>>;
}

pub fn driver<K: AsRef<str>, C: AsRef<str>>(kind: K, config: C) -> Result<Box<dyn OutputDriver>> {
    match kind.as_ref().to_lowercase().as_str() {
        "file" => Ok(Box::new(file::FileDriver::new(config.as_ref())?)),
        _ => bail!("unknown output driver kind"),
    }
}
