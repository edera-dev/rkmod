use rkmod::cache::InternCache;
use rkmod::controller::SystemModuleController;
use rkmod::directory::ModuleDirectory;
use rkmod::manager::ModuleManager;
use rkmod::util::normalize_module_name;

fn main() {
    let cache = InternCache::default();
    let directory =
        ModuleDirectory::current(cache.clone()).expect("failed to load module directory");
    let manager = ModuleManager::new(directory, SystemModuleController::default());
    let module = cache.get_string(normalize_module_name("batman-adv"));
    manager.probe(&[module]).expect("failed to probe module");
}
