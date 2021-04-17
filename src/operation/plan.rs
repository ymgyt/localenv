#[allow(unused_imports)]
use tracing::{debug, error, info, trace, warn};

use crate::{
    config::{Command, Config, Filesystem, FilesystemEntry},
    operation::{installer, Operation, OperationChain},
    prelude::*,
    system,
};
use nom::lib::std::collections::HashMap;

pub async fn plan<Api>(sys: Api, config: &Config) -> Result<OperationChain>
where
    Api: system::Api,
{
    let mut chain = OperationChain::new();

    plan_filesystem(&config.spec.filesystem, sys.os(), &mut chain).await?;
    plan_commands(config.spec.commands.as_slice(), &mut chain).await?;

    Ok(chain)
}

async fn plan_filesystem(
    fs: &Filesystem,
    sys_os: system::Os,
    chain: &mut OperationChain,
) -> Result<()> {
    let ops = fs
        .entries
        .iter()
        .flat_map(|entry| {
            trace!("{:?}", entry);

            // check condition
            if let Some(cond) = entry.condition() {
                // check os
                if let Some(os) = cond.os {
                    if os != sys_os {
                        debug!(
                            "entry {} does not match os condition. os: {}",
                            entry.description(),
                            os
                        );
                        return None;
                    }
                    debug!("entry {} match os condition", entry.description());
                }
            }

            let ops = match entry {
                FilesystemEntry::File(file) => Operation::create_file(file.clone()),
                FilesystemEntry::SymbolicLink(sym) => Operation::create_symbolic_link(sym.clone()),
                _ => unimplemented!(),
            };
            Some(ops)
        })
        .collect();

    chain.extend(ops);

    Ok(())
}

async fn plan_commands(commands: &[Command], chain: &mut OperationChain) -> Result<()> {
    use installer::Installer;

    // group by installer
    let h: HashMap<Installer, Vec<_>> = commands.iter().fold(HashMap::new(), |mut h, c| {
        match c.installer {
            Installer::Cargo => h
                .entry(installer::Installer::Cargo)
                .and_modify(|cmds| cmds.push(c.clone()))
                .or_insert_with(|| vec![c.clone()]),
        };
        h
    });

    for (inst, cmds) in h {
        match inst {
            Installer::Cargo => {
                let mut cargo = installer::Cargo::new()?;
                let installed_packages = cargo.list_installed_packages().await?;
                trace!("cargo installed packages: {:#?}", installed_packages);

                cmds.into_iter().for_each(|c| {
                    let installed = installed_packages.iter().any(|p| c.bin == p.bin());
                    if !installed {
                        chain.add(Operation::install_command(c));
                    } else {
                        debug!("{} already installed", c.bin);
                    }
                })
            }
        }
    }

    Ok(())
}
