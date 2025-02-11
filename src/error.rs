use std::sync::Arc;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("elf parse error: {0}")]
    ElfParse(#[from] elf::ParseError),
    #[error("io error: {0}")]
    Io(#[from] std::io::Error),
    #[error("nix error: {0}")]
    #[cfg(feature = "module-manager")]
    NixError(#[from] nix::errno::Errno),
    #[error("dependency loop found on {0}")]
    DependencyLoop(Arc<String>),
    #[error("dependency missing: {0}")]
    DependencyMissing(Arc<String>),
    #[error("unknown kernel release")]
    #[cfg(feature = "current-kernel")]
    UnknownKernelRelease,
}

pub type Result<T> = std::result::Result<T, Error>;
