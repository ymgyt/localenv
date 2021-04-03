use std::path::Path;

use crate::{config, operation, prelude::*, system};

/// Common terminate hook.
pub(super) fn exit(prefer_code: Option<i32>) {
    let code = prefer_code.unwrap_or(1);

    std::process::exit(code);
}

pub(super) async fn operation_chain(
    system: &mut system::System,
    config_dir: &Path,
) -> Result<(config::Config, operation::OperationChain)> {
    let config = config::Config::load_from_dir(config_dir).await?;

    debug!("load configuration from {}", config_dir.display());
    trace!("{:#?}", config);

    let ops_chain = operation::plan(system, &config).await?;

    trace!("planed operations {:#?}", ops_chain);

    Ok((config, ops_chain))
}
