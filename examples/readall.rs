use easy_parallel::Parallel;
use rkmod::cache::InternCache;
use rkmod::deps::ModuleDependencies;
use rkmod::ko::KernelObject;
use rkmod::textual::deps::TextualModuleDependencies;
use std::path::PathBuf;

fn main() {
    let path = std::env::args().nth(1).expect("path required");
    let path = PathBuf::from(path);

    let cache = InternCache::new();
    let mut deps = ModuleDependencies::new(cache.clone());
    TextualModuleDependencies::load(path.join("modules.dep"), &mut deps)
        .expect("failed to load dependencies");

    Parallel::new()
        .each(deps.all().keys(), |module| {
            let path = path.join(module.as_ref());
            let ko = KernelObject::open(path, cache.clone()).expect("failed to open module");
            let symbols = ko.dependency_symbols().expect("failed to fetch symbols");
            println!("{}: {} symbols", module.to_string_lossy(), symbols.len());
        })
        .run();
}
