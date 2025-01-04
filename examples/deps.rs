use rkmod::cache::InternCache;
use rkmod::database::ModuleDatabase;
use rkmod::textual::builtins::TextualModuleBuiltins;
use rkmod::textual::deps::TextualModuleDependencies;
use std::path::PathBuf;

fn main() {
    let path = std::env::args().nth(1).expect("path required");
    let path = PathBuf::from(path);

    let cache = InternCache::new();
    let mut database = ModuleDatabase::new(cache);
    TextualModuleDependencies::load(path.join("modules.dep"), &mut database)
        .expect("failed to load dependencies");
    TextualModuleBuiltins::load(path.join("modules.builtin"), &mut database)
        .expect("failed to load builtins");

    println!("{:?}", database);
}
