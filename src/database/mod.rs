use crate::cache::InternCache;
use crate::database::module::ModuleInfo;
use indexmap::IndexMap;
use std::sync::Arc;

pub mod dependency;
pub mod module;
pub mod resolution;
pub mod textual;
pub mod update;

#[derive(Clone, Debug)]
pub struct ModuleDatabase {
    modules: IndexMap<Arc<String>, ModuleInfo>,
    cache: InternCache,
}

impl ModuleDatabase {
    pub fn cache(&self) -> &InternCache {
        &self.cache
    }
}

impl ModuleDatabase {
    pub fn new(cache: InternCache) -> Self {
        Self {
            modules: IndexMap::new(),
            cache,
        }
    }

    pub fn modules(&self) -> &IndexMap<Arc<String>, ModuleInfo> {
        &self.modules
    }

    pub fn modules_mut(&mut self) -> &mut IndexMap<Arc<String>, ModuleInfo> {
        &mut self.modules
    }
}
