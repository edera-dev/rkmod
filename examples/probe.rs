use rkmod::cache::InternCache;
use rkmod::controller::SystemModuleController;
use rkmod::directory::ModuleDirectory;
use rkmod::manager::ModuleManager;
use rkmod::util::normalize_module_name;
use std::sync::Arc;

fn main() {
    let cache = InternCache::default();
    let modules: Vec<Arc<String>> = std::env::args()
        .skip(1)
        .map(|module| cache.get_string(normalize_module_name(module)))
        .collect();

    let directory =
        ModuleDirectory::current(cache.clone()).expect("failed to load module directory");
    let manager = ModuleManager::new(directory, SystemModuleController::default());
    manager.probe(&modules).expect("failed to probe modules");
}
