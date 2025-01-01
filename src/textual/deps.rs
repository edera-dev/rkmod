use crate::deps::ModuleDependencies;
use crate::error::Result;
use crate::util::open_file_bytes;
use bytes::Bytes;
use std::path::Path;

pub struct TextualModuleDependencies;

impl TextualModuleDependencies {
    pub fn parse(bytes: Bytes) -> ModuleDependencies {
        let string = String::from_utf8_lossy(&bytes);
        let mut deps = ModuleDependencies::new();
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

            let module_deps: Vec<String> = dep_list
                .split(" ")
                .filter(|dep| !dep.is_empty())
                .map(|dep| dep.trim().to_string())
                .collect();
            deps.insert(module.to_string(), module_deps);
        }
        deps
    }

    pub fn load(path: impl AsRef<Path>) -> Result<ModuleDependencies> {
        let bytes = open_file_bytes(path)?;
        Ok(TextualModuleDependencies::parse(bytes))
    }
}
