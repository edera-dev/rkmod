use crate::database::ModuleDatabase;
use crate::database::update::ModuleDatabaseUpdater;
use crate::error::Result;
use crate::util::{open_file_bytes, path_to_module_name};
use bstr::ByteSlice;
use bstr::io::BufReadExt;
use bytes::Bytes;
use std::ops::Deref;
use std::path::Path;

pub struct TextualModuleOrder;

impl TextualModuleOrder {
    pub fn parse(bytes: Bytes, database: &mut ModuleDatabase) -> Result<()> {
        let mut index = 0u64;
        bytes.deref().for_byte_line_with_terminator(|line| {
            if !line.starts_with(b"#") && !line.trim().is_empty() {
                let line = String::from_utf8_lossy(line);
                let module = path_to_module_name(&*line);
                let module = database.cache().get_string(module);
                ModuleDatabaseUpdater::set_order(database, module, index);
                index += 1;
            }
            Ok(true)
        })?;
        Ok(())
    }

    pub fn load(path: impl AsRef<Path>, database: &mut ModuleDatabase) -> Result<()> {
        let bytes = open_file_bytes(path)?;
        Self::parse(bytes, database)?;
        Ok(())
    }
}
