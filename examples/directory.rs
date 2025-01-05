use rkmod::cache::InternCache;
use rkmod::directory::ModuleDirectory;
use std::path::PathBuf;

fn main() {
    let root = std::env::args().nth(1).expect("path required");
    let cache = InternCache::default();
    let root = cache.get_path(PathBuf::from(root));
    let directory = ModuleDirectory::open(&*root, cache).expect("failed to open module directory");
    for (module, info) in directory.database().modules() {
        println!(
            "{} {}",
            module,
            info.path()
                .as_ref()
                .map(|path| path.to_string_lossy().to_string())
                .unwrap_or_default()
        );
    }
}
