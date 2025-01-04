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
    name: Arc<String>,
    form: ModuleDependencyForm,
}

impl ModuleDependency {
    pub fn new(name: Arc<String>, form: ModuleDependencyForm) -> Self {
        Self { name, form }
    }

    pub fn name(&self) -> &Arc<String> {
        &self.name
    }

    pub fn form(&self) -> &ModuleDependencyForm {
        &self.form
    }

    pub fn set_form(&mut self, form: ModuleDependencyForm) {
        self.form = form;
    }
}

#[derive(Eq, PartialEq, Hash, Clone, Debug)]
pub enum ModuleForm {
    Missing,
    File,
    Builtin,
}

#[derive(Clone, Debug)]
pub struct ModuleInfo {
    name: Arc<String>,
    dependencies: IndexMap<Arc<String>, ModuleDependency>,
    path: Option<Arc<PathBuf>>,
    form: ModuleForm,
}

impl ModuleInfo {
    pub fn new(name: Arc<String>) -> Self {
        Self {
            form: ModuleForm::Missing,
            name,
            dependencies: IndexMap::new(),
            path: None,
        }
    }

    pub fn name(&self) -> &Arc<String> {
        &self.name
    }

    pub fn path(&self) -> &Option<Arc<PathBuf>> {
        &self.path
    }

    pub fn dependencies(&self) -> &IndexMap<Arc<String>, ModuleDependency> {
        &self.dependencies
    }

    pub fn form(&self) -> &ModuleForm {
        &self.form
    }

    pub fn set_form(&mut self, form: ModuleForm) {
        self.form = form;
    }

    pub fn set_path(&mut self, path: Option<Arc<PathBuf>>) {
        self.path = path;
    }
}

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

    pub fn mark_builtin(&mut self, module: Arc<String>) {
        let info = self
            .modules
            .entry(module)
            .or_insert_with_key(|key| ModuleInfo::new(key.clone()));
        info.set_form(ModuleForm::Builtin);
    }

    pub fn update_dependencies(
        &mut self,
        module: Arc<String>,
        deps: Vec<ModuleDependency>,
        path: Option<Arc<PathBuf>>,
    ) {
        let info = self
            .modules
            .entry(module)
            .or_insert_with_key(|key| ModuleInfo::new(key.clone()));
        for dep in deps {
            info.dependencies
                .entry(dep.name().clone())
                .or_insert_with(|| ModuleDependency::new(dep.name().clone(), dep.form.clone()));
        }

        if path.is_some() {
            info.set_path(path);
            info.set_form(ModuleForm::File)
        }
    }
}
