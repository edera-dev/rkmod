use crate::database::ModuleDatabase;
use crate::database::dependency::{ModuleDependency, ModuleDependencyForm};
use crate::database::update::ModuleDatabaseUpdater;
use crate::error::Result;
use crate::util::{open_file_bytes, path_to_module_name};
use bstr::ByteSlice;
use bstr::io::BufReadExt;
use bytes::Bytes;
use std::ops::Deref;
use std::path::{Path, PathBuf};

pub struct TextualModuleDependencies;

impl TextualModuleDependencies {
    pub fn parse(bytes: Bytes, database: &mut ModuleDatabase) -> Result<()> {
        bytes.deref().for_byte_line_with_terminator(|line| {
            if line.starts_with_str(b"#") {
                return Ok(true);
            }

            if !line.contains_str(b":") {
                return Ok(true);
            }

            let Some((module, dep_list)) = line.split_once_str(b":") else {
                return Ok(true);
            };

            if module.is_empty() {
                return Ok(true);
            }

            let module = String::from_utf8_lossy(module);
            let path = PathBuf::from(&*module);
            let path = database.cache().get_path(path);
            let module = path_to_module_name(&*module);
            let module = database.cache().get_string(module);

            let module_deps: Vec<ModuleDependency> = dep_list
                .split_str(" ")
                .map(|dep| dep.trim())
                .filter(|dep| !dep.is_empty())
                .map(|dep| String::from_utf8_lossy(dep))
                .map(|dep| {
                    let dep = path_to_module_name(&*dep);
                    let dep = database.cache().get_string(dep);
                    ModuleDependency::new(dep, ModuleDependencyForm::Direct)
                })
                .collect();
            ModuleDatabaseUpdater::update_dependencies(database, module, module_deps, Some(path));
            Ok(true)
        })?;
        Ok(())
    }

    pub fn load(path: impl AsRef<Path>, database: &mut ModuleDatabase) -> Result<()> {
        let bytes = open_file_bytes(path)?;
        Self::parse(bytes, database)?;
        Ok(())
    }
}
