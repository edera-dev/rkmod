use rkmod::cache::InternCache;
use rkmod::deps::ModuleDatabase;
use rkmod::textual::deps::TextualModuleDependencies;
use std::path::PathBuf;

fn main() {
    let path = std::env::args().nth(1).expect("path required");
    let path = PathBuf::from(path);

    let cache = InternCache::new();
    let mut deps = ModuleDatabase::new(cache);
    TextualModuleDependencies::load(path.join("modules.dep"), &mut deps)
        .expect("failed to load dependencies");

    println!("{:?}", deps);
}
