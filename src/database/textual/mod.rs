use crate::database::ModuleDatabase;
use crate::database::textual::builtins::TextualModuleBuiltins;
use crate::database::textual::deps::TextualModuleDependencies;
use crate::database::textual::order::TextualModuleOrder;
use crate::error::Result;
use std::path::Path;

pub mod builtins;
pub mod deps;
pub mod order;

pub struct TextualModuleDatabase;

impl TextualModuleDatabase {
    pub fn load(path: impl AsRef<Path>, database: &mut ModuleDatabase) -> Result<()> {
        let path = path.as_ref();
        TextualModuleBuiltins::load(path.join("modules.builtin"), database)?;
        TextualModuleDependencies::load(path.join("modules.dep"), database)?;
        TextualModuleOrder::load(path.join("modules.order"), database)?;
        Ok(())
    }
}
