use crate::{config, prelude::Result};

#[derive(Debug)]
pub struct OperationChain {
    operations: Vec<Operation>,
}

impl OperationChain {
    pub(super) fn new() -> Self {
        Self {
            operations: Vec::new(),
        }
    }

    pub(super) fn add(&mut self, ops: Operation) {
        self.operations.push(ops)
    }

    pub(super) fn operations(&self) -> &[Operation] {
        self.operations.as_slice()
    }

    pub(super) fn operations_mut(&mut self) -> &mut [Operation] {
        self.operations.as_mut()
    }
}

#[derive(Debug)]
pub struct Operation<T = ()> {
    kind: OperationKind,
    result: Option<Result<T>>,
}

impl<T> Operation<T> {
    pub(super) fn kind(&self) -> &OperationKind {
        &self.kind
    }
    pub(super) fn set_result(&mut self, result: Result<T>) {
        self.result = Some(result);
    }

    pub(super) fn create_file(entry: config::FileEntry) -> Self {
        Operation::with(OperationKind::Filesystem(FilesystemOperation::CreateFile {
            entry,
        }))
    }

    pub(super) fn create_symbolic_link(entry: config::SymlinkEntry) -> Self {
        Operation::with(OperationKind::Filesystem(
            FilesystemOperation::CreateSymbolicLink { entry },
        ))
    }

    pub(super) fn install_command(cmd: config::Command) -> Self {
        Operation::with(OperationKind::Command(CommandOperation::Install { cmd }))
    }

    fn with(kind: OperationKind) -> Self {
        Self { kind, result: None }
    }
}

#[derive(Debug)]
pub enum OperationKind {
    Filesystem(FilesystemOperation),
    Command(CommandOperation),
}

#[derive(Debug)]
pub enum FilesystemOperation {
    CreateFile { entry: config::FileEntry },
    CreateSymbolicLink { entry: config::SymlinkEntry },
}

#[derive(Debug)]
pub enum CommandOperation {
    Install { cmd: config::Command },
}
