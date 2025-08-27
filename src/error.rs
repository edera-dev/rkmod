use std::sync::Arc;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("elf parse error: {0}")]
    #[cfg(feature = "elf")]
    ElfParse(#[from] elf::ParseError),
    #[error("io error: {0}")]
    Io(#[from] std::io::Error),
    #[error("nix error: {0}")]
    #[cfg(feature = "module-manager")]
    NixError(#[from] nix::errno::Errno),
    #[error("c string conversion error: {0}")]
    #[cfg(feature = "module-manager")]
    NulError(std::ffi::NulError),
    #[error("dependency loop found on {0}")]
    DependencyLoop(Arc<String>),
    #[error("dependency missing: {0}")]
    DependencyMissing(Arc<String>),
    #[error("unknown kernel release")]
    #[cfg(feature = "current-kernel")]
    UnknownKernelRelease,
    #[error("unsupported operation")]
    UnsupportedOperation,
    #[error("data decode error: {0}")]
    #[cfg(feature = "signature-core")]
    DataDecodeError(bytemuck::PodCastError),
}

pub type Result<T> = std::result::Result<T, Error>;
