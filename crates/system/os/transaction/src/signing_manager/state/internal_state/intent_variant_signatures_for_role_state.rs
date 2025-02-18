use crate::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct IntentVariantSignaturesForRoleState {
    role: RoleKind,
    signatures_per_entity:
        IndexMap<AddressOfAccountOrPersona, IndexSet<SignatureWithPublicKey>>,
}
impl IntentVariantSignaturesForRoleState {
    pub(crate) fn signatures_non_empty_map_with_non_empty_values(
        &self,
    ) -> Result<
        IndexMap<AddressOfAccountOrPersona, IndexSet<SignatureWithPublicKey>>,
    > {
        let signatures_per_entity = self
            .signatures_per_entity
            .clone()
            .into_iter()
            .filter(|(_, v)| !v.is_empty())
            .collect::<IndexMap<_, _>>();
        if signatures_per_entity.is_empty() {
            return Err(CommonError::Unknown); // TODO: Add error
        }

        Ok(self.signatures_per_entity.clone())
    }

    pub(crate) fn update_with_intent_with_signatures(
        &mut self,
        intent_with_signatures: EntitySignedFor,
    ) {
        assert_eq!(intent_with_signatures.role_kind(), self.role);
        self.signatures_per_entity.append_or_insert_to(
            intent_with_signatures.entity.address(),
            intent_with_signatures
                .intent_signatures()
                .into_iter()
                .map(|s| s.0)
                .collect_vec(),
        );
    }

    pub(crate) fn new(role: RoleKind) -> Self {
        Self {
            role,
            signatures_per_entity: IndexMap::new(),
        }
    }
}
