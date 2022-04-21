use std::path::PathBuf;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "logagg")]
pub struct CmdParser {
    /// Activate detach mode
    #[structopt(short, long)]
    pub detach: bool,

    /// Config file
    #[structopt(short, long, parse(from_os_str))]
    pub cfg: PathBuf,
}
