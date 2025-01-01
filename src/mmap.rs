use crate::error::Result;
use bytes::Bytes;
use memmap2::Mmap;
use std::fs::File;
use std::sync::Arc;

#[derive(Clone)]
pub struct OwnedMappedFile {
    #[allow(unused)]
    file: Arc<File>, // `mapped` field technically owns this
    mapped: Arc<Mmap>,
}

impl OwnedMappedFile {
    pub fn map(file: File) -> Result<Self> {
        let file = Arc::new(file);
        let mapped = unsafe { Mmap::map(&file)? };
        let mapped = Arc::new(mapped);
        Ok(OwnedMappedFile { file, mapped })
    }

    pub fn map_bytes(file: File) -> Result<Bytes> {
        let mapped = Self::map(file)?;
        Ok(Bytes::from_owner(mapped))
    }
}

impl AsRef<[u8]> for OwnedMappedFile {
    fn as_ref(&self) -> &[u8] {
        self.mapped.as_ref()
    }
}
