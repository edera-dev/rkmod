use thiserror::Error;

#[derive(Error, Debug)]
pub enum KmodError {
    #[error("elf parse error: {0}")]
    ElfParse(#[from] elf::ParseError),
    #[error("io error: {0}")]
    Io(#[from] std::io::Error),
    #[error("nix error: {0}")]
    NixError(#[from] nix::errno::Errno),
}

pub type Result<T> = std::result::Result<T, KmodError>;
