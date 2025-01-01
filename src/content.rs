use std::fs;
use crate::error::Result;
use bytes::{BufMut, Bytes, BytesMut};
use elf::{endian::LittleEndian, ElfBytes};
use std::io;
use std::path::Path;
use crate::{compression::CompressionFormat, util::check_magic_header};
#[cfg(feature = "file-mmap")]
use crate::mmap::OwnedMappedFile;

const ELF_MAGIC: &[u8] = &[
    0x7f, 0x45, 0x4c, 0x46, 0x02, 0x01, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
];

/// [KmodContents] provides a representation of .ko files in the kernel module tree.
/// It allows access to the raw contents while also making it easy to decompress
/// the compressed contents of files.
#[derive(Clone)]
pub struct KmodContents {
    /// The raw bytes of the kmod.
    bytes: Bytes,
}

impl KmodContents {
    /// Constructs a [KmodContents] from the specified [bytes].
    pub fn new(bytes: Bytes) -> Self {
        Self { bytes }
    }

    #[cfg(feature = "file-mmap")]
    pub fn open(path: impl AsRef<Path>) -> Result<Self> {
        let file = fs::File::open(path)?;
        let mapped = OwnedMappedFile::map(file)?;
        let bytes = Bytes::from_owner(mapped);
        Ok(Self::new(bytes))
    }
    
    #[cfg(not(feature = "file-mmap"))]
    pub fn open(path: impl AsRef<Path>) -> Result<Self> {
        let content = fs::read(path)?;
        Ok(Self::new(Bytes::from_owner(content)))
    }

    /// Borrows the file content bytes.
    pub fn bytes(&self) -> &Bytes {
        &self.bytes
    }

    /// Consumes [self] and returns the file content.
    pub fn into_bytes(self) -> Bytes {
        self.bytes
    }

    /// Checks if file contents is an ELF file.
    pub fn check_elf(&self) -> bool {
        check_magic_header(ELF_MAGIC, &self.bytes)
    }

    /// Parses the content as an ELF file.
    pub fn parse_elf(&self) -> Result<ElfBytes<LittleEndian>> {
        Ok(ElfBytes::minimal_parse(&self.bytes)?)
    }

    /// Determines the compression in use on the file content, if any.
    pub fn detect_compression(&self) -> Option<CompressionFormat> {
        CompressionFormat::detect(&self.bytes)
    }

    /// Decompress the byte contents, if needed.
    /// If compression is not in use, this method will return [Ok(self)[.
    /// The [CompressionFormat] is used to decompress, utilizing
    /// [Self::detect_compression] to figure out the compression format.
    pub fn decompress(self) -> Result<KmodContents> {
        let Some(compression) = self.detect_compression() else {
            return Ok(self);
        };

        let mut reader = compression.reader(self.into_bytes())?;
        let bytes = BytesMut::new();
        let mut writer = bytes.writer();
        io::copy(&mut reader, &mut writer)?;
        Ok(KmodContents::new(writer.into_inner().freeze()))
    }
}
