use crate::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq)]
struct IntentVariantSignaturesForRoleState {
    role: RoleKind,
    signatures_per_entity:
        IndexMap<AddressOfAccountOrPersona, IndexSet<SignatureWithPublicKey>>,
}
impl IntentVariantSignaturesForRoleState {
    fn update_with_intent_with_signatures(
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
    fn new(role: RoleKind) -> Self {
        Self {
            role,
            signatures_per_entity: IndexMap::new(),
        }
    }
}
