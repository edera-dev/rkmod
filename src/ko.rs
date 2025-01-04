use crate::cache::InternCache;
use crate::elf::ElfContent;
use crate::error::Result;
use crate::symbol::KernelSymbol;
use elf::abi::SHN_UNDEF;
use elf::symbol::Symbol;
use std::ffi::CStr;
use std::path::Path;

#[derive(Clone)]
pub struct KernelObject {
    content: ElfContent,
    cache: InternCache,
}

impl KernelObject {
    pub fn new(content: ElfContent, cache: InternCache) -> Self {
        Self { content, cache }
    }

    pub fn open(path: impl AsRef<Path>, cache: InternCache) -> Result<Self> {
        let contents = ElfContent::open(path)?.decompress()?;
        Ok(Self {
            content: contents,
            cache,
        })
    }

    pub fn collect_symbols(
        &self,
        accept: impl Fn(&Symbol, &str) -> bool,
    ) -> Result<Vec<KernelSymbol>> {
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
                let symbol = KernelSymbol::new(name);
                kernel_symbols.push(symbol);
            }
        }
        Ok(kernel_symbols)
    }

    pub fn dependency_symbols(&self) -> Result<Vec<KernelSymbol>> {
        self.collect_symbols(|symbol, symbol_name| {
            symbol.st_shndx == SHN_UNDEF && !symbol_name.is_empty()
        })
    }

    /// Inserts the module into the kernel with the specified command line.
    ///
    /// # Safety
    ///
    /// This function does not check anything about the module prior to insertion.
    /// Do not use this function directly unless you know what you are doing.
    pub unsafe fn insert_into_kernel(&self, cmdline: impl AsRef<CStr>) -> Result<()> {
        nix::kmod::init_module(self.content.bytes(), cmdline.as_ref())?;
        Ok(())
    }
}
