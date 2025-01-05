use std::sync::Arc;

#[derive(Eq, PartialEq, Hash, Clone, Debug, Ord, PartialOrd)]
pub enum ModuleDependencySoft {
    Normal,
    Pre,
    Post,
}

#[derive(Eq, PartialEq, Hash, Clone, Debug, Ord, PartialOrd)]
pub enum ModuleDependencyForm {
    Direct,
    Soft(ModuleDependencySoft),
    Weak,
}

#[derive(Eq, PartialEq, Hash, Clone, Debug)]
pub struct ModuleDependency {
    name: Arc<String>,
    form: ModuleDependencyForm,
}

impl ModuleDependency {
    pub fn new(name: Arc<String>, form: ModuleDependencyForm) -> Self {
        Self { name, form }
    }

    pub fn name(&self) -> &Arc<String> {
        &self.name
    }

    pub fn form(&self) -> &ModuleDependencyForm {
        &self.form
    }

    pub fn set_form(&mut self, form: ModuleDependencyForm) {
        self.form = form;
    }
}
