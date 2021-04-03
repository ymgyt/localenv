use std::fs;

use crate::{
    config::{Config, FileEntry, SymlinkEntry},
    operation::{FilesystemOperation, OperationChain, OperationKind},
    prelude::*,
    system,
};

pub struct ApplyParam<'cfg, 'ops, Api> {
    pub system: Api,
    pub config: &'cfg Config,
    pub operation_chain: &'ops mut OperationChain,
    pub dry_run: bool,
}

/// apply operations to system.
pub async fn apply<Api>(param: ApplyParam<'_, '_, Api>) -> Result<()>
where
    Api: system::Api,
{
    let ApplyParam {
        mut system,
        config,
        operation_chain,
        dry_run,
    } = param;

    for ops in operation_chain.operations_mut() {
        let result = match ops.kind() {
            OperationKind::Filesystem(fs) => match fs {
                FilesystemOperation::CreateFile { entry, .. } => {
                    apply_create_file_blocking(&mut system, config, dry_run, entry)
                }
                FilesystemOperation::CreateSymbolicLink { entry, .. } => {
                    apply_create_symbolic_link_blocking(&mut system, config, dry_run, entry)
                }
            },
        };

        ops.set_result(result);
    }

    Ok(())
}

fn apply_create_file_blocking<Api>(
    system: &mut Api,
    cfg: &Config,
    dry_run: bool,
    entry: &FileEntry,
) -> Result<()>
where
    Api: system::Api,
{
    let dest = entry.dest_path();
    let src = entry.src_path(cfg.root_dir.as_path());
    let mut content = fs::File::open(src.as_path())?;

    if dry_run {
        Ok(())
    } else {
        system.create_file(dest, &mut content, entry.permission()?)
    }
}

fn apply_create_symbolic_link_blocking<Api>(
    system: &mut Api,
    _cfg: &Config,
    dry_run: bool,
    entry: &SymlinkEntry,
) -> Result<()>
where
    Api: system::Api,
{
    let original = entry.original_path();
    let link = entry.link_path();

    if dry_run {
        Ok(())
    } else {
        system.create_symbolic_link(original, link)
    }
}
