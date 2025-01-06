use rkmod::cache::InternCache;
use rkmod::database::textual::TextualModuleDatabase;
use rkmod::database::ModuleDatabase;
use std::path::PathBuf;

fn main() {
    let path = std::env::args().nth(1).expect("path required");
    let path = PathBuf::from(path);
    let cache = InternCache::default();
    let mut database = ModuleDatabase::new(cache);
    TextualModuleDatabase::load(path, &mut database).expect("failed to load module database");
    for module in database.modules().keys() {
        println!("{}", module);
    }
}
