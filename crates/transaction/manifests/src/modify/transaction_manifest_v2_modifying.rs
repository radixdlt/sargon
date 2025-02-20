use crate::prelude::*;

pub trait TransactionManifestV2Modifying {
    fn modify_add_proofs(
        self,
        entities_with_access_controllers: IndexMap<
            AddressOfAccountOrPersona,
            AccessControllerAddress,
        >,
    ) -> Result<TransactionManifestV2>;
}

impl TransactionManifestV2Modifying for TransactionManifestV2 {
    fn modify_add_proofs(
        self,
        entities_with_access_controllers: IndexMap<
            AddressOfAccountOrPersona,
            AccessControllerAddress,
        >,
    ) -> Result<TransactionManifestV2> {
        self.modify_add_proofs_and_lock_fee(
            None,
            entities_with_access_controllers,
        )
    }
}

#[cfg(test)]
mod test {
    use super::*;
}
