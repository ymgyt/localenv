use std::fs;

use crate::{
    config::Config,
    operation::{FilesystemOperation, OperationChain, OperationKind},
    prelude::*,
    system,
};

pub async fn apply<Api>(mut system: Api, cfg: &Config, ops_chain: &OperationChain) -> Result<()>
where
    Api: system::Api,
{
    for ops in ops_chain.operations() {
        match ops.kind() {
            OperationKind::Filesystem(fs) => match fs {
                FilesystemOperation::CreateFile { entry, .. } => {
                    let dest = entry.dest_path();
                    let src = entry.src_path(cfg.root_dir.as_path());
                    let mut content = fs::File::open(src.as_path())?;

                    info!(dest = %(dest.display()), src = %(src.display()), "create file");
                    system.create_file(dest, &mut content)?;
                }
            },
        }
    }

    Ok(())
}
