use crate::elf::ElfContent;
use crate::error::Result;
use crate::symbol::KernelSymbol;
use elf::abi::SHN_UNDEF;
use elf::symbol::Symbol;
use std::path::Path;

#[derive(Clone)]
pub struct KernelObject {
    contents: ElfContent,
}

impl KernelObject {
    pub fn new(contents: ElfContent) -> Self {
        Self { contents }
    }

    pub fn open(path: impl AsRef<Path>) -> Result<Self> {
        let contents = ElfContent::open(path)?.decompress()?;
        Ok(Self { contents })
    }

    pub fn collect_symbols(
        &self,
        accept: impl Fn(&Symbol, &str) -> bool,
    ) -> Result<Vec<KernelSymbol>> {
        let elf = self.contents.read_elf()?;
        let Ok(Some((symbols, strtab))) = elf.symbol_table() else {
            return Ok(Vec::new());
        };

        let mut kernel_symbols = Vec::with_capacity(symbols.len());
        for symbol in symbols {
            let name = strtab.get(symbol.st_name as usize)?;

            if accept(&symbol, name) {
                let symbol = KernelSymbol::new(name.to_string());
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
}
