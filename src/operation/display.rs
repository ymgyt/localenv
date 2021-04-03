use colored::*;

use crate::{
    config::Config,
    operation::{FilesystemOperation, OperationChain, OperationKind},
    prelude::*,
    system,
};

pub struct DisplayParam<'cfg, 'ops, Api> {
    pub system: Api,
    pub config: &'cfg Config,
    pub operation_chain: &'ops OperationChain,
}

pub async fn display<Api>(param: DisplayParam<'_, '_, Api>) -> Result<()>
where
    Api: system::Api,
{
    let DisplayParam {
        system,
        config: _config,
        operation_chain,
    } = param;

    for ops in operation_chain.operations() {
        match ops.kind() {
            OperationKind::Filesystem(fs) => match fs {
                FilesystemOperation::CreateFile { entry, .. } => {
                    let dest = entry.dest_path();

                    let msg = format!(
                        "[Create file]\n    Desc: {}\n    File: {}",
                        entry.description(),
                        dest.display(),
                    );

                    system.display(msg.yellow());
                }
                FilesystemOperation::CreateSymbolicLink { entry, .. } => {
                    let original = entry.original_path();
                    let link = entry.link_path();

                    let msg = format!(
                        "[Create symlink]\n    Desc: {}\n    Orig: {}\n    Link: {}",
                        entry.description(),
                        original.display(),
                        link.display(),
                    );

                    system.display(msg.yellow());
                }
            },
        }
    }

    Ok(())
}
