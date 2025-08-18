use crate::error::Result;
use crate::util::open_file_bytes;
use crate::{compression::CompressionFormat, util::check_magic_header};
use bytes::{BufMut, Bytes, BytesMut};
use std::io;
use std::path::Path;

const ELF_MAGIC: &[u8] = &[0x7f, 0x45, 0x4c, 0x46];

/// [KernelObjectContent] provides a signature representation of .ko files in the kernel module tree.
/// It allows access to the signature contents while also making it easy to decompress
/// the compressed contents of files.
#[derive(Clone)]
pub struct KernelObjectContent {
    /// The signature bytes of the kmod.
    bytes: Bytes,
}

impl KernelObjectContent {
    /// Constructs a [KernelObjectContent] from the specified [bytes].
    pub fn new(bytes: Bytes) -> Self {
        Self { bytes }
    }

    pub fn open(path: impl AsRef<Path>) -> Result<Self> {
        let bytes = open_file_bytes(path)?;
        Ok(Self::new(bytes))
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
        // Bytes 1-4: EI_MAGIC, `\x7fELF` in ASCII
        if !check_magic_header(ELF_MAGIC, &self.bytes) {
            return false;
        }

        // Byte 5: EI_CLASS, 0x1 = 32-bit, 0x2 = 64-bit module
        if self.bytes[4] != 1 && self.bytes[4] != 2 {
            return false;
        }

        // Byte 6: EI_DATA (endianness), 0x1 = little, 0x2 = big
        if self.bytes[5] != 1 && self.bytes[5] != 2 {
            return false;
        }

        // Byte 7: EI_VERSION (ELF version), 0x1 is the only valid value
        if self.bytes[6] != 1 {
            return false;
        }

        true
    }

    /// Determines the compression in use on the file content, if any.
    pub fn detect_compression(&self) -> Option<CompressionFormat> {
        CompressionFormat::detect(&self.bytes)
    }

    /// Decompress the byte contents, if needed.
    /// If compression is not in use, this method will return [Ok(self)[.
    /// The [CompressionFormat] is used to decompress, utilizing
    /// [Self::detect_compression] to figure out the compression format.
    pub fn decompress(self) -> Result<KernelObjectContent> {
        let Some(compression) = self.detect_compression() else {
            return Ok(self);
        };

        let mut reader = compression.reader(self.into_bytes())?;
        let bytes = BytesMut::new();
        let mut writer = bytes.writer();
        io::copy(&mut reader, &mut writer)?;
        Ok(KernelObjectContent::new(writer.into_inner().freeze()))
    }
}

#[cfg(feature = "elf")]
impl KernelObjectContent {
    /// Parses the content as an ELF file.
    pub fn read_elf(&'_ self) -> Result<elf::ElfBytes<'_, elf::endian::AnyEndian>> {
        Ok(elf::ElfBytes::minimal_parse(&self.bytes)?)
    }
}
