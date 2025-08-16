use crate::database::ModuleDatabase;
use crate::database::dependency::ModuleDependency;
use crate::database::module::{ModuleForm, ModuleInfo};
use std::path::PathBuf;
use std::sync::Arc;

pub struct ModuleDatabaseUpdater;

impl ModuleDatabaseUpdater {
    pub fn mark_builtin(database: &mut ModuleDatabase, module: Arc<String>) {
        let info = database
            .modules_mut()
            .entry(module)
            .or_insert_with_key(|key| ModuleInfo::new(key.clone()));
        info.set_form(ModuleForm::Builtin);
    }

    pub fn update_dependencies(
        database: &mut ModuleDatabase,
        module: Arc<String>,
        deps: Vec<ModuleDependency>,
        path: Option<Arc<PathBuf>>,
    ) {
        let info = database
            .modules_mut()
            .entry(module)
            .or_insert_with_key(|key| ModuleInfo::new(key.clone()));
        for dep in deps {
            info.dependencies_mut()
                .entry(dep.name().clone())
                .or_insert_with(|| dep);
        }

        if let Some(path) = path {
            info.set_path(path);
            info.set_form(ModuleForm::File)
        }
    }

    pub fn set_order(database: &mut ModuleDatabase, module: Arc<String>, order: u64) {
        let info = database
            .modules_mut()
            .entry(module)
            .or_insert_with_key(|key| ModuleInfo::new(key.clone()));
        info.set_order(order);
    }
}
