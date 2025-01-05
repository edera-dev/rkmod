use rkmod::cache::InternCache;
use rkmod::object::KernelObject;

fn main() {
    let path = std::env::args().nth(1).expect("file path required");
    let ko =
        KernelObject::open(path, InternCache::default()).expect("failed to open kernel object");
    let symbols = ko.dependency_symbols().expect("failed to get symbols");
    for symbol in symbols {
        println!("symbol: {:?}", symbol.name());
    }
}
