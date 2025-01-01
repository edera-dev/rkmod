use indexmap::IndexMap;

pub struct ModuleDependencies {
    deps: IndexMap<String, Vec<String>>,
}

impl ModuleDependencies {
    pub fn all(&self) -> &IndexMap<String, Vec<String>> {
        &self.deps
    }
}

impl Default for ModuleDependencies {
    fn default() -> Self {
        Self::new()
    }
}

impl ModuleDependencies {
    pub fn new() -> Self {
        Self {
            deps: IndexMap::new(),
        }
    }

    pub fn insert(&mut self, module: String, deps: Vec<String>) {
        self.deps.insert(module, deps);
    }

    pub fn get(&self, module: &str) -> &[String] {
        self.deps.get(module).map(|dep| dep.as_ref()).unwrap_or(&[])
    }
}
