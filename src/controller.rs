use crate::error::Result;
use std::fs;
use std::path::PathBuf;

pub struct ModuleController {
    root: PathBuf,
}

impl Default for ModuleController {
    fn default() -> Self {
        Self::new(PathBuf::from("/sys/module"))
    }
}

impl ModuleController {
    pub fn new(root: impl Into<PathBuf>) -> Self {
        Self { root: root.into() }
    }

    pub fn root(&self) -> &PathBuf {
        &self.root
    }

    pub fn is_available(&self, module: impl AsRef<str>) -> Result<bool> {
        Ok(self.root.join(module.as_ref()).is_dir())
    }

    pub fn is_builtin(&self, module: impl AsRef<str>) -> Result<bool> {
        let module_dir = self.root.join(module.as_ref());
        Ok(module_dir.is_dir() && !module_dir.join("initstate").is_file())
    }

    pub fn is_live(&self, module: impl AsRef<str>) -> Result<bool> {
        let module_dir = self.root.join(module.as_ref());
        if !module_dir.is_dir() {
            return Ok(false);
        }
        let initstate_path = module_dir.join("initstate");
        if !initstate_path.is_file() {
            return Ok(true);
        }
        Ok(fs::read_to_string(&initstate_path)?.trim() == "live")
    }
}
