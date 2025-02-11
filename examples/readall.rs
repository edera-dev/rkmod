use easy_parallel::Parallel;
use rkmod::cache::InternCache;
use rkmod::database::textual::builtins::TextualModuleBuiltins;
use rkmod::database::textual::deps::TextualModuleDependencies;
use rkmod::database::ModuleDatabase;
use rkmod::object::KernelObject;
use std::path::PathBuf;

fn main() {
    let path = std::env::args().nth(1).expect("path required");
    let root = PathBuf::from(path);

    let cache = InternCache::default();
    let mut database = ModuleDatabase::new(cache.clone());
    TextualModuleDependencies::load(root.join("modules.dep"), &mut database)
        .expect("failed to load dependencies");
    TextualModuleBuiltins::load(root.join("modules.builtin"), &mut database)
        .expect("failed to load builtins");

    Parallel::new()
        .each(database.modules().values(), |module| {
            let Some(path) = module.path() else {
                return;
            };

            let full = root.join(path.as_path());
            let ko = KernelObject::open(full, cache.clone()).expect("failed to open module");
            let symbols = ko.dependency_symbols().expect("failed to fetch symbols");
            println!("{}: {} symbols", module.name(), symbols.len());
        })
        .run();
}
