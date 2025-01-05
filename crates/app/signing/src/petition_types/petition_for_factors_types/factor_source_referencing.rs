use crate::prelude::*;

impl<ID: SignableID> FactorSourceReferencing for HDSignature<ID> {
    fn factor_source_id(&self) -> FactorSourceIDFromHash {
        self.owned_factor_instance()
            .factor_instance()
            .factor_source_id
    }
}
