use crate::prelude::*;

pub trait OsNewFactorAdding {
    fn make_add_new_factor_builder(
        &self,
        factor_kind: FactorSourceKind,
    ) -> AddNewFactorBuilder;
}

impl OsNewFactorAdding for Arc<SargonOS> {
    fn make_add_new_factor_builder(
        &self,
        factor_kind: FactorSourceKind,
    ) -> AddNewFactorBuilder {
        AddNewFactorBuilder::new(Arc::clone(self), factor_kind)
    }
}
