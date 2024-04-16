#[macro_use]
extern crate anyhow;

use crate::cfg::Cfg;
use anyhow::{Context, Result};
use std::path::PathBuf;
use structopt::StructOpt;

mod cfg;
mod output;
mod server;

#[tokio::main]
async fn main() -> Result<()> {
    simple_logger::init_with_level(log::Level::Debug)?;
    let cmd = CmdArgs::from_args();
    let cfg = Cfg::new(cmd.cfg).context("failed to load server config")?;

    server::server(cfg).await?;
    Ok(())
}

#[derive(StructOpt, Debug)]
#[structopt(name = "logagg")]
pub struct CmdArgs {
    /// Config file
    #[structopt(short, long, parse(from_os_str))]
    pub cfg: PathBuf,
}
