use std::fmt::Debug;
use std::path::PathBuf;
use std::sync::Arc;

#[derive(Clone)]
pub struct InternCache {
    #[cfg(feature = "cache-intern")]
    strings: Option<Arc<dashmap::DashMap<Arc<String>, Arc<String>>>>,
    #[cfg(feature = "cache-intern")]
    paths: Option<Arc<dashmap::DashMap<Arc<PathBuf>, Arc<PathBuf>>>>,
}

impl Default for InternCache {
    #[cfg(feature = "cache-intern")]
    fn default() -> Self {
        InternCache::cached()
    }

    #[cfg(not(feature = "cache-intern"))]
    fn default() -> Self {
        InternCache::uncached()
    }
}

impl InternCache {
    #[cfg(feature = "cache-intern")]
    pub fn cached() -> Self {
        Self {
            strings: Some(Arc::new(dashmap::DashMap::new())),
            paths: Some(Arc::new(dashmap::DashMap::new())),
        }
    }

    #[cfg(feature = "cache-intern")]
    pub fn uncached() -> Self {
        Self {
            strings: None,
            paths: None,
        }
    }

    #[cfg(not(feature = "cache-intern"))]
    pub fn uncached() -> Self {
        Self {}
    }

    #[cfg(feature = "cache-intern")]
    pub fn get_string(&self, key: String) -> Arc<String> {
        let key = Arc::new(key);
        match self.strings.as_ref() {
            Some(strings) => strings.entry(key.clone()).or_insert(key).clone(),
            None => key,
        }
    }

    #[cfg(not(feature = "cache-intern"))]
    pub fn get_string(&self, key: String) -> Arc<String> {
        Arc::new(key)
    }

    #[cfg(feature = "cache-intern")]
    pub fn get_path(&self, key: PathBuf) -> Arc<PathBuf> {
        let key = Arc::new(key);
        match self.paths.as_ref() {
            Some(paths) => paths.entry(key.clone()).or_insert(key).clone(),
            None => key,
        }
    }

    #[cfg(not(feature = "cache-intern"))]
    pub fn get_path(&self, key: PathBuf) -> Arc<PathBuf> {
        Arc::new(key)
    }
}

impl Debug for InternCache {
    #[cfg(feature = "cache-intern")]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match (self.strings.as_ref(), self.paths.as_ref()) {
            (Some(strings), Some(paths)) => {
                write!(
                    f,
                    "InternCache {{ strings = {}, paths = {} }}",
                    strings.len(),
                    paths.len(),
                )
            }
            _ => {
                write!(f, "InternCache {{ }}")
            }
        }
    }

    #[cfg(not(feature = "cache-intern"))]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "InternCache {{ }}")
    }
}
