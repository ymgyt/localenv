use crate::config;

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
}

#[derive(Debug)]
pub struct Operation {
    kind: OperationKind,
}

impl Operation {
    pub(super) fn kind(&self) -> &OperationKind {
        &self.kind
    }

    pub(super) fn create_file(entry: config::FileEntry) -> Self {
        Operation::with(OperationKind::Filesystem(FilesystemOperation::CreateFile {
            entry,
        }))
    }

    pub (super) fn create_symbolic_link(entry: config::SymlinkEntry) -> Self {
        Operation::with(OperationKind::Filesystem(FilesystemOperation::CreateSymbolicLink {
            entry,
        }))
    }

    fn with(kind: OperationKind) -> Self {
        Self { kind }
    }
}

#[derive(Debug)]
pub enum OperationKind {
    Filesystem(FilesystemOperation),
}

#[derive(Debug)]
pub enum FilesystemOperation {
    CreateFile { entry: config::FileEntry },
    CreateSymbolicLink { entry: config::SymlinkEntry },
}
