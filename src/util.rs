use crate::error::Result;
use crate::mmap::OwnedMappedFile;
use bytes::Bytes;
use std::fs;
use std::path::Path;

pub fn check_magic_header(magic: &[u8], data: &[u8]) -> bool {
    if data.len() < magic.len() {
        return false;
    }
    magic.iter().enumerate().all(|(i, byte)| *byte == data[i])
}

#[cfg(feature = "file-mmap")]
pub fn open_file_bytes(path: impl AsRef<Path>) -> Result<Bytes> {
    let file = fs::File::open(path)?;
    let mapped = OwnedMappedFile::map(file)?;
    let bytes = Bytes::from_owner(mapped);
    Ok(bytes)
}

#[cfg(not(feature = "file-mmap"))]
pub fn open_file_bytes(path: impl AsRef<Path>) -> Result<Bytes> {
    let content = fs::read(path)?;
    Ok(Bytes::from_owner(content))
}
