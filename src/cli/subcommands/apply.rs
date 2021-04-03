use std::path::PathBuf;

use structopt::StructOpt;

use crate::{cli::helper, operation, prelude::*, system::System};

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
        helper::exit(None);
    }
}

async fn apply(opt: Apply) -> Result<()> {
    let mut system = System::new();
    let (config, mut ops_chain) =
        helper::operation_chain(&mut system, opt.config_dir_path.as_path())
            .await
            .context("running apply")?;

    operation::apply(operation::ApplyParam {
        system: &mut system,
        config: &config,
        operation_chain: &mut ops_chain,
        dry_run: opt.dry_run,
    })
    .await?;

    operation::display(operation::DisplayParam{
        operation_chain: &ops_chain,
        system: &mut system,
        config: &config,
    }).await?;

    Ok(())
}
