use crate::controller::SystemModuleController;
use crate::database::directory::ModuleDirectory;
use crate::database::module::ModuleForm;
use crate::database::resolution::ModuleResolutionSet;
use crate::error::{Error, Result};
use std::ffi::CString;
use std::sync::Arc;

pub struct ModuleManager {
    directory: ModuleDirectory,
    controller: SystemModuleController,
}

impl ModuleManager {
    pub fn new(directory: ModuleDirectory, controller: SystemModuleController) -> Self {
        Self {
            directory,
            controller,
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
            unsafe {
                object.insert_into_kernel(CString::default())?;
            };
        }

        Ok(())
    }
}
