use crate::cache::InternCache;
use crate::database::textual::TextualModuleDatabase;
use crate::database::ModuleDatabase;
use crate::error::{Error, Result};
use crate::object::KernelObject;
use crate::util::current_kernel_release;
use std::path::{Path, PathBuf};
use std::sync::Arc;

#[derive(Clone)]
pub struct ModuleDirectory {
    root: Arc<PathBuf>,
    database: ModuleDatabase,
}

impl ModuleDirectory {
    pub fn new(root: Arc<PathBuf>, database: ModuleDatabase) -> Self {
        Self { root, database }
    }

    pub fn root(&self) -> &PathBuf {
        &self.root
    }

    pub fn database(&self) -> &ModuleDatabase {
        &self.database
    }

    pub fn resolve_path(&self, path: impl AsRef<Path>) -> Arc<PathBuf> {
        let path = path.as_ref();
        self.database.cache().get_path(self.root.join(path))
    }

    pub fn open_object_by_path(&self, path: impl AsRef<Path>) -> Result<KernelObject> {
        KernelObject::open(&*self.resolve_path(path), self.database.cache().clone())
    }

    pub fn open(root: impl AsRef<Path>, cache: InternCache) -> Result<Self> {
        let mut database = ModuleDatabase::new(cache);
        TextualModuleDatabase::load(root.as_ref(), &mut database)?;
        Ok(Self::new(
            database.cache().get_path(root.as_ref().to_path_buf()),
            database,
        ))
    }

    const LIB_MODULES_PATH: &'static str = "/lib/modules";

    #[cfg(feature = "current-kernel")]
    pub fn current(cache: InternCache) -> Result<Self> {
        let current_kernel_release =
            cache.get_string(current_kernel_release().ok_or(Error::UnknownKernelRelease)?);
        let root = PathBuf::from(Self::LIB_MODULES_PATH).join(&*current_kernel_release);
        Self::open(root, cache)
    }
}
