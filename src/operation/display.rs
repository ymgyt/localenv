use colored::*;

use crate::{
    config::Config,
    operation::{CommandOperation, FilesystemOperation, OperationChain, OperationKind},
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
            OperationKind::Command(cmd_ops) => match cmd_ops {
                CommandOperation::Install { cmd, .. } => {
                    let msg = format!(
                        "[Install command]\n     Bin: {}\n     Ver: {}\n    From: {:?}",
                        &cmd.bin, &cmd.version, &cmd.installer,
                    );

                    system.display(msg.yellow());
                }
            },
        }

        if let Some(result) = ops.result() {
            use std::borrow::Cow;
            let result: Cow<str> = match result {
                Ok(_) => "Success".into(),
                Err(err) => format!("{:?}", err.kind()).into(),
            };
            let msg = format!("  Result: {}", result);
            system.display(msg.yellow());
        }
    }

    Ok(())
}
