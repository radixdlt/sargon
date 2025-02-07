use crate::prelude::*;

pub struct AddNewFactorBuilder {
    os_ref: Arc<SargonOS>,
    factor_source_kind: FactorSourceKind,
    name: RwLock<String>,
}

impl AddNewFactorBuilder {
    pub fn new(
        os_ref: Arc<SargonOS>,
        factor_source_kind: FactorSourceKind,
    ) -> Self {
        Self {
            os_ref,
            factor_source_kind,
            name: RwLock::new("".to_owned()),
        }
    }
}

impl PartialEq for AddNewFactorBuilder {
    fn eq(&self, other: &Self) -> bool {
        self.factor_source_kind == other.factor_source_kind
    }
}

impl Eq for AddNewFactorBuilder {}

impl Clone for AddNewFactorBuilder {
    fn clone(&self) -> Self {
        Self {
            os_ref: self.os_ref.clone(),
            factor_source_kind: self.factor_source_kind,
            name: RwLock::new(
                self.name.read().expect("Failed to read name").clone(),
            ),
        }
    }
}

impl std::hash::Hash for AddNewFactorBuilder {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.factor_source_kind.hash(state);
    }
}

// ====================
// ==== GET / READ ====
// ====================
impl AddNewFactorBuilder {
    pub fn get_name(&self) -> String {
        self.name.read().unwrap().clone()
    }
}

// ====================
// ===== MUTATION =====
// ====================
impl AddNewFactorBuilder {
    pub fn set_name(&self, name: impl AsRef<str>) -> &Self {
        *self.name.write().unwrap() = name.as_ref().to_owned();
        self
    }
}
