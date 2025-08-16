use rkmod::error::Result;

#[cfg(all(feature = "module-manager", target_os = "linux"))]
fn main() -> Result<()> {
    use rkmod::cache::InternCache;
    use rkmod::object::KernelObject;
    use std::ffi::CString;
    use std::path::Path;

    let args = std::env::args().skip(1).collect::<Vec<_>>();
    let path = args.first().expect("module path required");
    let path = Path::new(&path);
    let cache = InternCache::cached();
    let object = KernelObject::open(path, cache)?;
    let cmdline = args[1..].join(" ");
    let cmdline = CString::new(cmdline).expect("failed to construct module command line");
    unsafe {
        object.insert_into_kernel(cmdline)?;
    }
    Ok(())
}

#[cfg(not(all(feature = "module-manager", target_os = "linux")))]
fn main() -> Result<()> {
    use rkmod::error::Error;

    Err(Error::UnsupportedOperation)
}
