use crate::deps::{ModuleDatabase, ModuleDependency, ModuleDependencyForm};
use crate::error::Result;
use crate::util::{open_file_bytes, path_to_module_name};
use bytes::Bytes;
use std::path::{Path, PathBuf};

pub struct TextualModuleDependencies;

impl TextualModuleDependencies {
    pub fn parse(bytes: Bytes, database: &mut ModuleDatabase) {
        let string = String::from_utf8_lossy(&bytes);
        for line in string.split("\n") {
            if !line.contains(":") {
                continue;
            }
            let Some((module, dep_list)) = line.split_once(":") else {
                continue;
            };

            if module.is_empty() {
                continue;
            }

            let path = PathBuf::from(module);
            let path = database.cache().get_path(path);
            let module = path_to_module_name(module);
            let module = database.cache().get_string(module);

            let module_deps: Vec<ModuleDependency> = dep_list
                .split(" ")
                .filter(|dep| !dep.is_empty())
                .map(|dep| dep.trim().to_string())
                .map(|dep| {
                    let dep = path_to_module_name(dep);
                    let dep = database.cache().get_string(dep);
                    ModuleDependency::new(dep, ModuleDependencyForm::Direct)
                })
                .collect();
            database.update_dependencies(module, module_deps, Some(path));
        }
    }

    pub fn load(path: impl AsRef<Path>, database: &mut ModuleDatabase) -> Result<()> {
        let bytes = open_file_bytes(path)?;
        Self::parse(bytes, database);
        Ok(())
    }
}
