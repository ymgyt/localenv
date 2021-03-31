use std::path::PathBuf;

use structopt::StructOpt;

use crate::prelude::*;

const APPLY_ABOUT: &str = "\
about apply subcommand...
";

#[derive(StructOpt, Debug, Clone)]
#[structopt(about = APPLY_ABOUT)]
pub struct Apply {
    #[structopt(long = "dir", help = "configuration directory path to apply.")]
    pub config_dir_path: PathBuf, // NOTE: required or default current directory.
}

pub async fn run(opt: Apply) {
    info!("running...")
}
