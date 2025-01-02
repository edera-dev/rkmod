use dashmap::DashMap;
use std::fmt::Debug;
use std::path::PathBuf;
use std::sync::Arc;

#[derive(Clone, Default)]
pub struct InternCache {
    #[cfg(feature = "intern-cache")]
    strings: Arc<DashMap<Arc<String>, Arc<String>>>,
    #[cfg(feature = "intern-cache")]
    paths: Arc<DashMap<Arc<PathBuf>, Arc<PathBuf>>>,
}

impl InternCache {
    pub fn new() -> Self {
        Self::default()
    }

    #[cfg(feature = "intern-cache")]
    pub fn get_string(&self, key: String) -> Arc<String> {
        let key = Arc::new(key);
        self.strings.entry(key.clone()).or_insert(key).clone()
    }

    #[cfg(not(feature = "intern-cache"))]
    pub fn get_string(&self, key: String) -> Arc<String> {
        Arc::new(key)
    }

    #[cfg(feature = "intern-cache")]
    pub fn get_path(&self, key: PathBuf) -> Arc<PathBuf> {
        let key = Arc::new(key);
        self.paths.entry(key.clone()).or_insert(key).clone()
    }

    #[cfg(not(feature = "intern-cache"))]
    pub fn get_path(&self, key: PathBuf) -> Arc<PathBuf> {
        Arc::new(key)
    }
}

impl Debug for InternCache {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "InternCache {{ strings = {}, paths = {} }}",
            self.strings.len(),
            self.paths.len()
        )
    }
}
