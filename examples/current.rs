use rkmod::cache::InternCache;
use rkmod::directory::ModuleDirectory;

fn main() {
    let directory =
        ModuleDirectory::current(InternCache::default()).expect("could not load module directory");
    println!(
        "{} has {} modules",
        directory.root().to_string_lossy(),
        directory.database().modules().len()
    );
}
