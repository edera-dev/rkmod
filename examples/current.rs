use rkmod::cache::InternCache;
use rkmod::controller::SystemModuleController;
use rkmod::directory::ModuleDirectory;

fn main() {
    let directory =
        ModuleDirectory::current(InternCache::default()).expect("could not load module directory");
    let controller = SystemModuleController::default();
    let mut loaded: usize = 0;
    for module in directory.database().modules().values() {
        let is_live = controller.is_live(module.name().as_str()).unwrap_or(false);

        if is_live {
            loaded += 1;
        }
    }

    println!(
        "{} has {} modules ({} loaded)",
        directory.root().to_string_lossy(),
        directory.database().modules().len(),
        loaded,
    );
}
