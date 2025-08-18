use std::sync::Arc;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct KernelSymbol {
    name: Arc<String>,
}

impl KernelSymbol {
    pub fn new(name: Arc<String>) -> KernelSymbol {
        KernelSymbol { name }
    }

    pub fn name(&self) -> &Arc<String> {
        &self.name
    }
}
