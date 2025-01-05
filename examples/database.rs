use rkmod::cache::InternCache;
use rkmod::database::resolution::ModuleResolutionSet;
use rkmod::database::textual::TextualModuleDatabase;
use rkmod::database::ModuleDatabase;
use std::path::PathBuf;

fn main() {
    let path = std::env::args().nth(1).expect("path required");
    let module = std::env::args().nth(2).expect("module name required");
    let path = PathBuf::from(path);
    let cache = InternCache::default();
    let mut database = ModuleDatabase::new(cache);
    TextualModuleDatabase::load(path, &mut database).expect("failed to load module database");
    let module = database.cache().get_string(module);
    let mut set = ModuleResolutionSet::new();
    set.add(&database, module).expect("failed to add module");
    for module in set.modules() {
        let info = database
            .modules()
            .get(module)
            .expect("failed to get module in set");
        println!(
            "name={} path={:?} order={:?}",
            module,
            info.path(),
            info.order()
        );
    }
}
