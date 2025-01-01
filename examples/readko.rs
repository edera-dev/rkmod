use rkmod::ko::KernelObject;

fn main() {
    let path = std::env::args().nth(1).expect("file path required");
    let ko = KernelObject::open(path).expect("failed to open kernel object");
    let symbols = ko.dependency_symbols().expect("failed to get symbols");
    for symbol in symbols {
        println!("symbol: {:?}", symbol);
    }
}
