use rkmod::cache::InternCache;
use rkmod::controller::ModuleController;
use rkmod::database::resolution::ModuleResolutionSet;
use rkmod::directory::ModuleDirectory;
use rkmod::util::normalize_module_name;

fn main() {
    let modules = std::env::args().skip(1).collect::<Vec<_>>();
    let directory =
        ModuleDirectory::current(InternCache::default()).expect("Could not load module directory");
    let modules = modules
        .into_iter()
        .map(normalize_module_name)
        .map(|module| directory.database().cache().get_string(module))
        .collect::<Vec<_>>();
    let mut resolution = ModuleResolutionSet::new();
    for module in modules {
        resolution
            .add(directory.database(), module)
            .expect("failed to add module");
    }
    let controller = ModuleController::default();
    for module in resolution.modules() {
        if controller
            .is_live(module.as_ref())
            .expect("failed to check for live module")
        {
            println!("live {}", module);
            continue;
        }
        println!("load {}", module);
    }
}
