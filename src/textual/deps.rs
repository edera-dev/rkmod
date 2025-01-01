use crate::deps::ModuleDependencies;
use crate::error::Result;
use crate::util::open_file_bytes;
use bytes::Bytes;
use std::path::Path;

pub struct TextualModuleDependencies;

impl TextualModuleDependencies {
    pub fn parse(bytes: Bytes, deps: &mut ModuleDependencies) {
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

            let module_deps: Vec<String> = dep_list
                .split(" ")
                .filter(|dep| !dep.is_empty())
                .map(|dep| dep.trim().to_string())
                .collect();
            deps.insert(module.to_string(), module_deps);
        }
    }

    pub fn load(path: impl AsRef<Path>, deps: &mut ModuleDependencies) -> Result<()> {
        let bytes = open_file_bytes(path)?;
        Self::parse(bytes, deps);
        Ok(())
    }
}
