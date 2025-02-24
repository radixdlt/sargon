use crate::prelude::*;

#[derive(Clone, PartialEq, Eq, derive_more::Debug)]
#[debug("#{} entities w sigs", self.signatures_per_entity.len())]
pub(crate) struct IntentVariantSignaturesForRoleState {
    /// Not present when initialized, but set when the first entity is added.
    /// and then used to assert that all entities have the same intent.
    intent: Option<TransactionIntentHash>,
    role: Immutable<RoleKind>,

    /// A bit confusing perhaps since you dont skip a entity, you skip a factor,
    /// perhaps this should be transposed to be a map of factors to entities...
    neglected_factors_per_entity:
        IndexMap<AddressOfAccountOrPersona, IndexSet<NeglectedFactor>>,

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

    fn set_intent_else_assert(&mut self, intent: &TransactionIntent) {
        if let Some(existing_intent) = self.intent.as_ref() {
            assert_eq!(*existing_intent, intent.transaction_intent_hash());
        } else {
            self.intent = Some(intent.transaction_intent_hash());
        }
    }

    pub(crate) fn update_with_entity_not_signed_for(
        &mut self,
        not_signed: EntityNotSignedFor,
    ) {
        assert_eq!(not_signed.role_kind(), *self.role);
        self.set_intent_else_assert(&not_signed.intent);
        self.neglected_factors_per_entity.append_or_insert_to(
            not_signed.entity.address(),
            not_signed.neglected_factor_sources(),
        );
    }

    pub(crate) fn update_with_intent_with_signatures(
        &mut self,
        intent_with_signatures: EntitySignedFor,
    ) {
        assert_eq!(intent_with_signatures.role_kind(), *self.role);
        self.set_intent_else_assert(&intent_with_signatures.intent);
        self.signatures_per_entity.append_or_insert_to(
            intent_with_signatures.entity.address(),
            intent_with_signatures
                .intent_signatures()
                .into_iter()
                .map(|s| s.0)
                .collect_vec(),
        );
    }

    pub(crate) fn new(role: impl Into<Immutable<RoleKind>>) -> Self {
        Self {
            intent: None,
            role: role.into(),
            signatures_per_entity: IndexMap::new(),
            neglected_factors_per_entity: IndexMap::new(),
        }
    }
}
