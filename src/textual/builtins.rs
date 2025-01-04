use crate::database::ModuleDatabase;
use crate::error::Result;
use crate::util::{open_file_bytes, path_to_module_name};
use bytes::Bytes;
use std::path::Path;

pub struct TextualModuleBuiltins;

impl TextualModuleBuiltins {
    pub fn parse(bytes: Bytes, database: &mut ModuleDatabase) -> Result<()> {
        let string = String::from_utf8_lossy(&bytes);
        for line in string.lines() {
            let line = line.trim();
            if line.is_empty() {
                continue;
            }
            let module = path_to_module_name(line);
            let module = database.cache().get_string(module);
            database.mark_builtin(module);
        }
        Ok(())
    }

    pub fn load(path: impl AsRef<Path>, database: &mut ModuleDatabase) -> Result<()> {
        let bytes = open_file_bytes(path)?;
        Self::parse(bytes, database)?;
        Ok(())
    }
}
