use crate::{
    config::{Config, FilesystemEntry},
    operation::{Operation, OperationChain},
    prelude::*,
    system,
};

pub async fn plan<Api>(mut _system: Api, config: &Config) -> Result<OperationChain>
where
    Api: system::Api,
{
    let mut chain = OperationChain::new();

    // filesystem
    config
        .spec
        .filesystem
        .entries
        .iter()
        .inspect(|(name, entry)| trace!("{} {:?}", name, entry))
        .for_each(|(_name, entry)| match entry {
            FilesystemEntry::File(file) => {
                let ops = Operation::create_file(file.clone());
                chain.add(ops);
            }
            _ => unimplemented!(),
        });

    Ok(chain)
}
