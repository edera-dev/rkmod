use crate::cache::InternCache;
use indexmap::IndexMap;
use std::path::PathBuf;
use std::sync::Arc;

#[derive(Eq, PartialEq, Hash, Clone, Debug, Ord, PartialOrd)]
pub enum ModuleDependencySoft {
    Normal,
    Pre,
    Post,
}

#[derive(Eq, PartialEq, Hash, Clone, Debug, Ord, PartialOrd)]
pub enum ModuleDependencyForm {
    Direct,
    Soft(ModuleDependencySoft),
    Weak,
}

#[derive(Eq, PartialEq, Hash, Clone, Debug)]
pub struct ModuleDependency {
    path: Arc<PathBuf>,
    form: ModuleDependencyForm,
}

impl ModuleDependency {
    pub fn new(path: Arc<PathBuf>, form: ModuleDependencyForm) -> Self {
        Self { path, form }
    }

    pub fn path(&self) -> &Arc<PathBuf> {
        &self.path
    }

    pub fn form(&self) -> &ModuleDependencyForm {
        &self.form
    }

    pub fn set_form(&mut self, form: ModuleDependencyForm) {
        self.form = form;
    }
}

#[derive(Clone, Debug)]
pub struct ModuleDependencies {
    dependencies: IndexMap<Arc<PathBuf>, IndexMap<Arc<PathBuf>, ModuleDependency>>,
    cache: InternCache,
}

impl ModuleDependencies {
    pub fn all(&self) -> &IndexMap<Arc<PathBuf>, IndexMap<Arc<PathBuf>, ModuleDependency>> {
        &self.dependencies
    }

    pub fn cache(&self) -> &InternCache {
        &self.cache
    }
}

impl ModuleDependencies {
    pub fn new(cache: InternCache) -> Self {
        Self {
            dependencies: IndexMap::new(),
            cache,
        }
    }

    pub fn insert(&mut self, module: Arc<PathBuf>, deps: Vec<ModuleDependency>) {
        let dependencies = self.dependencies.entry(module).or_default();
        for dep in deps {
            dependencies
                .entry(dep.path.clone())
                .or_insert_with(|| ModuleDependency::new(dep.path().clone(), dep.form.clone()));
        }
    }

    pub fn get(&self, module: &Arc<PathBuf>) -> impl Iterator<Item = &ModuleDependency> {
        self.dependencies
            .get(module)
            .map(|dep| dep.values())
            .unwrap_or_default()
    }
}
