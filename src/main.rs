extern crate yaml_rust;
use crate::{cfg::Cfg, cmd::CmdParser};
use structopt::StructOpt;
use tokio::{fs::OpenOptions, io::AsyncWriteExt};

mod cfg;
mod cmd;
mod output;
mod server;

fn main() {
    let cmd = CmdParser::from_args();
    let cfg = Cfg::new(cmd.cfg).unwrap();

    let rt = actix_rt::Runtime::new().unwrap();
    rt.block_on(async {
        let _ = server::server(cfg).await.unwrap();
    });
}
