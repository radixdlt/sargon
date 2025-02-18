use crate::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct UnsecurifiedIntentSetInternalState {
    /// N.B. this is silly in the context of UnsecurifiedIntentSetInternalState since
    /// for Unsecurified entities we only have Primary role. But for sake of consistency
    /// with SecurifiedIntentSetInternalState we keep it here - since other DTOs
    /// and states of SigningManager require this field in the case of Securified entities.
    pub(crate) intent_set_id: Immutable<IntentSetID>,

    account_paying_for_transaction: Immutable<ApplicationInputPayingAccount>,
    pub(crate) entity_applying_shield: Immutable<AnyUnsecurifiedEntity>,
    pub(crate) transaction_intent: Immutable<TransactionIntent>,

    signatures: IntentVariantSignaturesForRoleState,
}

impl UnsecurifiedIntentSetInternalState {
    fn role_kind(&self) -> RoleKind {
        // For unsecurified entities we only have Primary role.
        RoleKind::Primary
    }
    fn variant(
        &self,
    ) -> Option<RolesExercisableInTransactionManifestCombination> {
        // For unsecurified entities we dont have many variants.
        None
    }

    pub(crate) fn get_signatures(&self) -> Result<EntitySignedFor> {
        let entity = self.entity_applying_shield.entity.clone();
        self.transaction_intent.validate_required_signers_are([
            entity.address(),
            self.account_paying_for_transaction.account_address().into(),
        ])?;
        let outcome = self
            .signatures
            .signatures_non_empty_map_with_non_empty_values()?;
        let outcome = outcome
            .get(&entity.address())
            .ok_or(CommonError::Unknown)
            .cloned()?; // TODO better error

        Ok(EntitySignedFor::new(
            EntitySigningContext::new(
                *self.intent_set_id,
                self.role_kind(),
                self.variant(),
            ),
            (*self.transaction_intent).clone(),
            entity,
            outcome,
        ))
    }

    pub(crate) fn paying_account(&self) -> Account {
        self.account_paying_for_transaction.account()
    }

    pub(crate) fn transaction_intent_hash(&self) -> TransactionIntentHash {
        self.transaction_intent.transaction_intent_hash()
    }

    pub(crate) fn update_with_intent_with_signatures(
        &mut self,
        intent_with_signatures: EntitySignedFor,
    ) {
        assert_eq!(intent_with_signatures.intent, *self.transaction_intent);
        assert_eq!(
            intent_with_signatures.entity.address(),
            self.entity_applying_shield.address()
        );

        self.signatures
            .update_with_intent_with_signatures(intent_with_signatures);
    }
    fn new(
        intent_set_id: impl Into<Immutable<IntentSetID>>,
        account_paying_for_transaction: impl Into<
            Immutable<ApplicationInputPayingAccount>,
        >,
        entity_applying_shield: impl Into<Immutable<AnyUnsecurifiedEntity>>,
        transaction_intent: impl Into<Immutable<TransactionIntent>>,
    ) -> Self {
        Self {
            intent_set_id: intent_set_id.into(),
            account_paying_for_transaction: account_paying_for_transaction
                .into(),
            entity_applying_shield: entity_applying_shield.into(),
            transaction_intent: transaction_intent.into(),
            // For unsecurified entities we only have Primary role.
            signatures: IntentVariantSignaturesForRoleState::new(
                RoleKind::Primary,
            ),
        }
    }
}
impl
    From<(
        SecurityShieldApplicationForUnsecurifiedEntityWithTransactionIntent,
        IntentSetID,
    )> for UnsecurifiedIntentSetInternalState
{
    fn from(
        (application_with_intent, intent_set_id): (
            SecurityShieldApplicationForUnsecurifiedEntityWithTransactionIntent,
            IntentSetID,
        ),
    ) -> Self {
        Self::new(
            intent_set_id,
            application_with_intent.paying_account(),
            application_with_intent.entity_applying_shield(),
            application_with_intent.transaction_intent(),
        )
    }
}
