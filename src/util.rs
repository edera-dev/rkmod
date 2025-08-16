use crate::error::Result;
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
    let mapped = crate::mmap::OwnedMappedFile::map(file)?;
    let bytes = Bytes::from_owner(mapped);
    Ok(bytes)
}

#[cfg(not(feature = "file-mmap"))]
pub fn open_file_bytes(path: impl AsRef<Path>) -> Result<Bytes> {
    let content = fs::read(path)?;
    Ok(Bytes::from_owner(content))
}

pub fn normalize_module_name(name: impl Into<String>) -> String {
    name.into().replace("-", "_")
}

pub fn path_to_module_name(path: impl AsRef<Path>) -> String {
    let path = path.as_ref();
    normalize_module_name(
        path.file_name()
            .map(|name| name.to_string_lossy())
            .map(|name| {
                name.split_once(".")
                    .map(|(first, _second)| first.to_string())
                    .unwrap_or_else(|| name.to_string())
            })
            .unwrap_or_default(),
    )
}

#[cfg(feature = "current-kernel")]
#[cfg(target_os = "linux")]
pub fn current_kernel_release() -> Option<String> {
    // workaround for clippy trying to be helpful...
    trait CanRepresentU8 {
        fn as_u8(&self) -> u8;
    }

    impl CanRepresentU8 for u8 {
        fn as_u8(&self) -> u8 {
            *self
        }
    }

    impl CanRepresentU8 for i8 {
        fn as_u8(&self) -> u8 {
            *self as u8
        }
    }

    unsafe {
        let mut uts: libc::utsname = std::mem::zeroed();
        let _ = libc::uname(std::ptr::addr_of_mut!(uts));
        let release_bytes: Vec<u8> = uts
            .release
            .into_iter()
            .take_while(|x| *x != 0)
            .map(|x| x.as_u8())
            .collect();
        String::from_utf8(release_bytes).ok()
    }
}
