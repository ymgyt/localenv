use std::path::PathBuf;

use structopt::StructOpt;

use crate::{cli::helper, operation, prelude::*, system::System};

const PLAN_ABOUT: &str = "\
about plan subcommand...
";

#[derive(StructOpt, Debug, Clone)]
#[structopt(about = PLAN_ABOUT)]
pub struct Plan {
    #[structopt(long = "dir", help = "configuration directory path to apply.")]
    pub config_dir_path: PathBuf, // NOTE: required or default current directory.
}

pub async fn run(opt: Plan) {
    // Validate opt if needed.
    if let Err(err) = plan(opt).await {
        error!("{}", err);
        helper::exit(None);
    }
}

async fn plan(opt: Plan) -> Result<()> {
    let mut system = System::new();
    let (config, ops_chain) =
        helper::operation_chain(&mut system, opt.config_dir_path.as_path()).await?;

    operation::display(operation::DisplayParam {
        system: &mut system,
        config: &config,
        operation_chain: &ops_chain,
    })
    .await?;

    Ok(())
}
