use crate::cfg::Cfg;
use std::path::PathBuf;
use structopt::StructOpt;

mod cfg;
mod output;
mod server;

fn main() {
    simple_logger::init().unwrap();
    let cmd = CmdArgs::from_args();
    let cfg = Cfg::new(cmd.cfg).unwrap();

    let rt = actix_rt::Runtime::new().unwrap();
    rt.block_on(async {
        let _ = server::server(cfg).await.unwrap();
    });
}

#[derive(StructOpt, Debug)]
#[structopt(name = "logagg")]
pub struct CmdArgs {
    /// Activate detach mode
    #[structopt(short, long)]
    pub detach: bool,

    /// Config file
    #[structopt(short, long, parse(from_os_str))]
    pub cfg: PathBuf,
}

