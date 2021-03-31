use std::path::PathBuf;

use structopt::StructOpt;

use crate::{cli, config, prelude::*};

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
    // Validate opt if needed.
    if let Err(err) = apply(opt).await {
        error!("{}", err);
        cli::exit(None);
    }
}

async fn apply(opt: Apply) -> Result<()> {
    let config = config::Config::load_from_dir(&opt.config_dir_path).await?;

    debug!("load configuration {:#?}", config);

    Ok(())
}
