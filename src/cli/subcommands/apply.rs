use std::path::PathBuf;

use structopt::StructOpt;

use crate::{cli, config, operation, prelude::*, system::System};

const APPLY_ABOUT: &str = "\
about apply subcommand...
";

#[derive(StructOpt, Debug, Clone)]
#[structopt(about = APPLY_ABOUT)]
pub struct Apply {
    #[structopt(long = "dir", help = "configuration directory path to apply.")]
    pub config_dir_path: PathBuf, // NOTE: required or default current directory.

    #[structopt(long = "dry-run", help = "no changed will occur in dry run mode.")]
    pub dry_run: bool,
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

    let mut system = System::new();

    let ops_chain = operation::plan(&mut system, &config).await?;
    debug!("planed operations {:#?}", ops_chain);

    operation::apply(operation::ApplyParam {
        system: &mut system,
        config: &config,
        operation_chain: &ops_chain,
        dry_run: opt.dry_run,
    })
    .await?;

    Ok(())
}
