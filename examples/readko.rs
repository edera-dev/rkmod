use rkmod::cache::InternCache;
use rkmod::object::KernelObject;
use rkmod::signature::raw::RawKernelObjectSignature;

fn main() {
    let path = std::env::args().nth(1).expect("file path required");
    let ko =
        KernelObject::open(path, InternCache::default()).expect("failed to open kernel object");
    let symbols_used = ko.dependency_symbols().expect("failed to get symbols");
    println!("{} symbols used", symbols_used.len());
    let signature =
        RawKernelObjectSignature::load(ko.content().bytes()).expect("failed to load signature");

    if let Some(signature) = signature {
        println!("signature:");
        println!("  length: {}", signature.signature.len());
        if !signature.key_id.is_empty() {
            println!("  key id: {}", signature.key_id_str());
        }

        if !signature.signer.is_empty() {
            println!("  signer: {}", signature.signer_str());
        }
    }
}
