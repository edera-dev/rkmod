use crate::database::dependency::ModuleDependency;
use indexmap::IndexMap;
use std::path::PathBuf;
use std::sync::Arc;

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
    order: Option<u64>,
}

impl ModuleInfo {
    pub fn new(name: Arc<String>) -> Self {
        Self {
            form: ModuleForm::Missing,
            name,
            dependencies: IndexMap::new(),
            path: None,
            order: None,
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

    pub fn dependencies_mut(&mut self) -> &mut IndexMap<Arc<String>, ModuleDependency> {
        &mut self.dependencies
    }

    pub fn form(&self) -> &ModuleForm {
        &self.form
    }

    pub fn order(&self) -> &Option<u64> {
        &self.order
    }

    pub fn set_form(&mut self, form: ModuleForm) {
        self.form = form;
    }

    pub fn set_path(&mut self, path: Arc<PathBuf>) {
        self.path = Some(path);
    }

    pub fn set_order(&mut self, order: u64) {
        self.order = Some(order);
    }
}
