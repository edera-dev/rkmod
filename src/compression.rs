#[cfg(feature = "compression-core")]
use bytes::Buf;
use bytes::Bytes;
use std::io::Read;

use crate::error::Result;
use crate::util::check_magic_header;

#[cfg(feature = "compression-gzip")]
const GZIP_MAGIC: &[u8] = &[0x1f, 0x8b];

#[cfg(feature = "compression-xz2")]
const XZ2_MAGIC: &[u8] = &[0xfd, 0x37, 0x7a, 0x58];

#[cfg(feature = "compression-zlib")]
const ZLIB_MAGIC: &[u8] = &[0x78];

#[cfg(feature = "compression-zstd")]
const ZSTD_MAGIC: &[u8] = &[0x28, 0xb5, 0x2f, 0xfd];

#[derive(Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub enum CompressionFormat {
    #[cfg(feature = "compression-gzip")]
    Gzip,
    #[cfg(feature = "compression-xz2")]
    Xz2,
    #[cfg(feature = "compression-zstd")]
    Zstd,
    #[cfg(feature = "compression-zlib")]
    Zlib,
}

impl CompressionFormat {
    pub const FORMATS: &'static [CompressionFormat] = &[
        #[cfg(feature = "compression-gzip")]
        CompressionFormat::Gzip,
        #[cfg(feature = "compression-xz2")]
        CompressionFormat::Xz2,
        #[cfg(feature = "compression-zlib")]
        CompressionFormat::Zlib,
        #[cfg(feature = "compression-zstd")]
        CompressionFormat::Zstd,
    ];

    #[cfg(feature = "compression-core")]
    pub fn magic(&self) -> &'static [u8] {
        match self {
            #[cfg(feature = "compression-gzip")]
            Self::Gzip => GZIP_MAGIC,
            #[cfg(feature = "compression-xz2")]
            Self::Xz2 => XZ2_MAGIC,
            #[cfg(feature = "compression-zlib")]
            Self::Zlib => ZLIB_MAGIC,
            #[cfg(feature = "compression-zstd")]
            Self::Zstd => ZSTD_MAGIC,
        }
    }

    #[cfg(not(feature = "compression-core"))]
    pub fn magic(&self) -> &'static [u8] {
        unreachable!()
    }

    pub fn has_magic(&self, data: &[u8]) -> bool {
        check_magic_header(self.magic(), data)
    }

    pub fn detect(data: &[u8]) -> Option<CompressionFormat> {
        for format in Self::FORMATS {
            if data.len() < format.magic().len() {
                continue;
            }
            if format.has_magic(data) {
                return Some(format.clone());
            }
        }
        None
    }

    #[cfg(feature = "compression-core")]
    pub fn reader(&self, bytes: Bytes) -> Result<Box<dyn Read>> {
        let reader = bytes.reader();
        Ok(match self {
            #[cfg(feature = "compression-gzip")]
            Self::Gzip => Box::new(flate2::bufread::GzDecoder::new(reader)) as Box<dyn Read>,

            #[cfg(feature = "compression-xz2")]
            Self::Xz2 => Box::new(xz2::bufread::XzDecoder::new(reader)) as Box<dyn Read>,

            #[cfg(feature = "compression-zlib")]
            Self::Zlib => Box::new(flate2::bufread::ZlibDecoder::new(reader)) as Box<dyn Read>,

            #[cfg(feature = "compression-zstd")]
            Self::Zstd => Box::new(zstd::stream::read::Decoder::new(reader)?) as Box<dyn Read>,
        })
    }

    #[cfg(not(feature = "compression-core"))]
    pub fn reader(&self, _bytes: Bytes) -> Result<Box<dyn Read>> {
        unreachable!()
    }
}
