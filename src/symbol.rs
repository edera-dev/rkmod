#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct KernelSymbol {
    name: String,
}

impl KernelSymbol {
    pub fn new(name: String) -> KernelSymbol {
        KernelSymbol { name }
    }

    pub fn name(&self) -> &str {
        &self.name
    }
}
