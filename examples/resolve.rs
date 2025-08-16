use rkmod::cache::InternCache;
use rkmod::database::resolution::ModuleResolutionSet;
use rkmod::directory::ModuleDirectory;
use rkmod::error::Result;
use rkmod::util::normalize_module_name;
use std::path::Path;

#[cfg(target_os = "linux")]
fn is_live(module: &str) -> Result<bool> {
    rkmod::controller::SystemModuleController::default().is_live(module)
}

#[cfg(not(target_os = "linux"))]
fn is_live(_module: &str) -> Result<bool> {
    Ok(false)
}

fn main() {
    let modules = std::env::args().skip(1).collect::<Vec<_>>();
    let env_modules_path = std::env::var("MODULES_DIR").ok();
    let directory = if let Some(env_modules_path) = env_modules_path {
        let modules_path = Path::new(&env_modules_path);
        ModuleDirectory::open(modules_path, InternCache::default())
            .expect("Could not load module directory")
    } else {
        #[cfg(not(all(target_os = "linux", feature = "current-kernel")))]
        panic!("current modules directory is only supported on Linux with current-kernel");
        #[cfg(all(target_os = "linux", feature = "current-kernel"))]
        ModuleDirectory::current(InternCache::default()).expect("Could not load module directory")
    };
    let modules = modules
        .into_iter()
        .map(normalize_module_name)
        .map(|module| directory.database().cache().get_string(module))
        .collect::<Vec<_>>();
    let mut resolution = ModuleResolutionSet::new();
    for module in modules {
        resolution
            .add(directory.database(), module)
            .expect("failed to add module");
    }
    for module in resolution.modules() {
        if is_live(module).expect("failed to check for live module") {
            println!("live {module}");
            continue;
        }
        println!("load {module}");
    }
}
