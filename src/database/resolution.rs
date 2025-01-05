use crate::database::ModuleDatabase;
use crate::error::{Error, Result};
use std::collections::BTreeSet;
use std::sync::Arc;

#[derive(Clone, Debug, Default)]
pub struct ModuleResolutionSet {
    modules: Vec<Arc<String>>,
}

impl ModuleResolutionSet {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add(&mut self, database: &ModuleDatabase, module: Arc<String>) -> Result<()> {
        let mut resolving = BTreeSet::new();
        self.add_module(database, module, &mut resolving)?;
        Ok(())
    }

    fn add_module(
        &mut self,
        database: &ModuleDatabase,
        module: Arc<String>,
        resolving: &mut BTreeSet<Arc<String>>,
    ) -> Result<()> {
        if self.modules.contains(&module) {
            return Ok(());
        }
        if resolving.contains(&module) {
            return Err(Error::DependencyLoop(module));
        }
        resolving.insert(module.clone());

        let info = database
            .modules()
            .get(&module)
            .ok_or_else(|| Error::DependencyMissing(module.clone()))?;
        for (dependency, _depinfo) in info.dependencies() {
            self.add_module(database, dependency.clone(), resolving)?;
        }
        resolving.remove(&module);
        self.modules.push(module);
        Ok(())
    }

    pub fn modules(&self) -> &Vec<Arc<String>> {
        &self.modules
    }

    pub fn into_modules(self) -> Vec<Arc<String>> {
        self.modules
    }
}
