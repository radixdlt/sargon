use crate::prelude::*;
use sargon::OsNewFactorAdding;

#[derive(Hash, PartialEq, Clone, uniffi::Object)]
#[uniffi::export(Hash, Eq)]
pub struct AddNewFactorBuilder {
    wrapped: Arc<sargon::AddNewFactorBuilder>,
}

#[uniffi::export]
impl SargonOS {
    pub fn make_add_new_factor_builder(
        &self,
        factor_source_kind: FactorSourceKind,
    ) -> AddNewFactorBuilder {
        let builder = self
            .wrapped
            .make_add_new_factor_builder(factor_source_kind.into_internal());
        AddNewFactorBuilder {
            wrapped: Arc::new(builder),
        }
    }
}

impl AddNewFactorBuilder {
    fn get<R>(&self, access: impl Fn(&sargon::AddNewFactorBuilder) -> R) -> R {
        let binding = self.wrapped.clone();
        access(&binding)
    }

    fn set(
        self: Arc<Self>,
        write: impl Fn(
            &Arc<sargon::AddNewFactorBuilder>,
        ) -> &sargon::AddNewFactorBuilder,
    ) -> Arc<Self> {
        builder_arc_map(self, |builder| {
            _ = write(&builder.wrapped);
        })
    }
}

// ====================
// ==== GET / READ ====
// ====================
#[uniffi::export]
impl AddNewFactorBuilder {
    pub fn get_name(&self) -> String {
        self.get(|builder| builder.get_name())
    }
}

// ====================
// ===== MUTATION =====
// ====================
#[uniffi::export]
impl AddNewFactorBuilder {
    pub fn set_name(self: Arc<Self>, name: String) -> Arc<Self> {
        self.set(|builder| builder.set_name(&name))
    }
}
