use crate::controller::SystemModuleController;
use crate::database::directory::ModuleDirectory;
use crate::database::module::ModuleForm;
use crate::database::resolution::ModuleResolutionSet;
use crate::error::{Error, Result};
use std::ffi::CString;
use std::sync::Arc;

use std::collections::HashMap;

#[derive(Clone, Debug)]
pub struct ModuleParameterSet {
    parameters: HashMap<Arc<String>, Vec<Arc<String>>>,
}

impl ModuleParameterSet {
    pub fn new() -> Self {
        Self {
            parameters: HashMap::new(),
        }
    }

    pub fn all(&self) -> &HashMap<Arc<String>, Vec<Arc<String>>> {
        &self.parameters
    }

    pub fn get(&self, module: &Arc<String>) -> Option<&Vec<Arc<String>>> {
        self.parameters.get(module)
    }

    pub fn insert(&mut self, module: Arc<String>, parameters: &[Arc<String>]) {
        self.parameters
            .entry(module)
            .or_default()
            .extend_from_slice(parameters);
    }

    pub fn replace(&mut self, module: Arc<String>, parameters: Vec<Arc<String>>) {
        self.parameters.insert(module, parameters);
    }
}

impl Default for ModuleParameterSet {
    fn default() -> Self {
        ModuleParameterSet::new()
    }
}

pub struct ModuleManager {
    directory: ModuleDirectory,
    controller: SystemModuleController,
    parameters: ModuleParameterSet,
}

impl ModuleManager {
    pub fn new(
        directory: ModuleDirectory,
        controller: SystemModuleController,
        parameters: ModuleParameterSet,
    ) -> Self {
        Self {
            directory,
            controller,
            parameters,
        }
    }

    pub fn probe(&self, modules: &[Arc<String>]) -> Result<()> {
        let mut set = ModuleResolutionSet::new();
        for module in modules {
            set.add(self.directory.database(), module.clone())?;
        }

        for module in set.modules() {
            if self.controller.is_live(module.as_str())? {
                continue;
            }

            let info = self
                .directory
                .database()
                .modules()
                .get(module)
                .ok_or(Error::DependencyMissing(module.clone()))?;
            if *info.form() != ModuleForm::File {
                continue;
            }

            let Some(path) = info.path() else {
                continue;
            };

            let object = self.directory.open_object_by_path(path.as_path())?;

            let parameters = if let Some(parameters) = self.parameters.get(module) {
                CString::new(
                    parameters
                        .iter()
                        .map(|item| item.to_string())
                        .collect::<Vec<_>>()
                        .join(" "),
                )
                .map_err(Error::NulError)?
            } else {
                CString::default()
            };

            unsafe {
                object.insert_into_kernel(parameters)?;
            };
        }

        Ok(())
    }
}
