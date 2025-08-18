use crate::cache::InternCache;
use crate::error::Result;
use crate::object::content::KernelObjectContent;
use std::path::Path;

/// content.rs: provide signature file content.rs for kmods
///
/// One interesting challenge of interacting with kernel modules is
/// that they can be compressed or just plain ELF files. The APIs
/// in this Rust module provide access to signature file contents using
/// the [bytes] crate, which provides cheaply cloneable access to
/// bytes.
pub mod content;
#[cfg(feature = "elf")]
pub mod symbol;

#[derive(Clone)]
pub struct KernelObject {
    content: KernelObjectContent,
    #[allow(unused)]
    cache: InternCache,
}

impl KernelObject {
    pub fn new(content: KernelObjectContent, cache: InternCache) -> Self {
        Self { content, cache }
    }

    pub fn open(path: impl AsRef<Path>, cache: InternCache) -> Result<Self> {
        let contents = KernelObjectContent::open(path)?.decompress()?;
        Ok(Self {
            content: contents,
            cache,
        })
    }

    pub fn content(&self) -> &KernelObjectContent {
        &self.content
    }
}

#[cfg(feature = "elf")]
impl KernelObject {
    pub fn collect_symbols(
        &self,
        accept: impl Fn(&elf::symbol::Symbol, &str) -> bool,
    ) -> Result<Vec<symbol::KernelSymbol>> {
        let elf = self.content.read_elf()?;
        let Ok(Some((symbols, strtab))) = elf.symbol_table() else {
            return Ok(Vec::new());
        };

        let mut kernel_symbols = Vec::with_capacity(symbols.len());
        for symbol in symbols {
            let name = strtab.get(symbol.st_name as usize)?;

            if accept(&symbol, name) {
                let name = name.to_string();
                let name = self.cache.get_string(name);
                let symbol = symbol::KernelSymbol::new(name);
                kernel_symbols.push(symbol);
            }
        }
        Ok(kernel_symbols)
    }

    pub fn dependency_symbols(&self) -> Result<Vec<symbol::KernelSymbol>> {
        self.collect_symbols(|symbol, symbol_name| {
            symbol.st_shndx == elf::abi::SHN_UNDEF && !symbol_name.is_empty()
        })
    }
}

#[cfg(all(feature = "module-manager", target_os = "linux"))]
impl KernelObject {
    /// Inserts the module into the kernel with the specified command line.
    ///
    /// # Safety
    ///
    /// This function does not check anything about the module prior to insertion.
    /// Do not use this function directly unless you know what you are doing.
    pub unsafe fn insert_into_kernel(&self, cmdline: impl AsRef<std::ffi::CStr>) -> Result<()> {
        nix::kmod::init_module(self.content.bytes(), cmdline.as_ref())?;
        Ok(())
    }
}
