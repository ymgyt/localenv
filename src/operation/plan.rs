use crate::{
    config::{Config, FilesystemEntry},
    operation::{Operation, OperationChain},
    prelude::*,
    system,
};

pub async fn plan<Api>(sys: Api, config: &Config) -> Result<OperationChain>
where
    Api: system::Api,
{
    let mut chain = OperationChain::new();

    // filesystem
    config.spec.filesystem.entries.iter().for_each(|entry| {
        trace!("{:?}", entry);

        // check condition
        if let Some(cond) = entry.condition() {
            // check os
            if let Some(os) = cond.os {
                if os != sys.os() {
                    debug!(
                        "entry {} does not match os condition. os: {}",
                        entry.description(),
                        os
                    );
                    return;
                }
                debug!("entry {} match os condition", entry.description());
            }
        }

        let ops = match entry {
            FilesystemEntry::File(file) => Operation::create_file(file.clone()),
            FilesystemEntry::SymbolicLink(sym) => Operation::create_symbolic_link(sym.clone()),
            _ => unimplemented!(),
        };

        chain.add(ops);
    });

    // commands
    config.spec.commands.iter().for_each(|cmd| {
        let ops = Operation::install_command(cmd.clone());

        chain.add(ops);
    });

    Ok(chain)
}
