use crate::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq)]
#[allow(clippy::large_enum_variant)]
pub(crate) enum IntentSetInternalState {
    Unsecurified(UnsecurifiedIntentSetInternalState),
    Securified(SecurifiedIntentSetInternalState),
}
impl IntentSetInternalState {
    pub(crate) fn get_signed_intents(
        &self,
    ) -> Result<Vec<EntitySignedForWithVariant>> {
        match self {
            Self::Unsecurified(unsec) => {
                unsec.get_signatures().map(|sig| vec![sig])
            }
            Self::Securified(sec) => sec.get_signed_intents(),
        }
    }

    /// Returns `None` is `Unsecurified`
    /// Returns `Some(self.entity_applying_shield)` if that entity has not been signed for
    /// with Recovery role
    /// Returns `None` otherwise.
    pub(crate) fn entities_not_signed_for_with_recovery(
        &self,
    ) -> Option<AccountOrPersona> {
        match self {
            Self::Unsecurified(_) => None,
            Self::Securified(sec) => {
                if sec.has_exercised_recovery_role_for_entity_applying_shield()
                {
                    None
                } else {
                    Some(self.entity_applying_shield())
                }
            }
        }
    }

    // Returns `None` is `Unsecurified`
    /// Returns `Some(self.entity_applying_shield)` if that entity has been signed for
    /// with Recovery role but not with confirmation
    /// Returns `None` otherwise.
    pub(crate) fn entities_signed_for_with_recovery_but_not_with_confirmation(
        &self,
    ) -> Option<AccountOrPersona> {
        match self {
            Self::Unsecurified(_) => None,
            Self::Securified(sec) => {
                if sec.entities_signed_for_with_recovery_but_not_with_confirmation()
                {
                    None
                } else {
                    Some(self.entity_applying_shield())
                }
            }
        }
    }

    /// will return `False` if Self is `Unsecurified`, since unsecured entities only have primary role
    pub(crate) fn has_exercised_recovery_and_confirmation_role_for_all_entities(
        &self,
    ) -> bool {
        match self {
            Self::Unsecurified(_) => {
                // Unsecurified entities only have primary role
                false
            }
            Self::Securified(sec) => sec
                .has_exercised_recovery_and_confirmation_role_for_all_entities(
                ),
        }
    }

    pub(crate) fn entity_applying_shield(&self) -> AccountOrPersona {
        match self {
            Self::Unsecurified(unsec) => unsec.entity_applying_shield().entity,
            Self::Securified(sec) => sec.entity_applying_shield().entity,
        }
    }

    pub(crate) fn paying_account(&self) -> Account {
        match self {
            Self::Unsecurified(unsec) => unsec.paying_account(),
            Self::Securified(sec) => sec.paying_account(),
        }
    }

    pub(crate) fn transaction_intent_hashes(
        &self,
    ) -> IndexSet<TransactionIntentHash> {
        match self {
            Self::Unsecurified(unsec) => {
                IndexSet::just(unsec.transaction_intent_hash())
            }
            Self::Securified(sec) => sec.transaction_intent_hashes(),
        }
    }

    pub(crate) fn can_exercise_role(&self, role_kind: RoleKind) -> bool {
        match self {
            Self::Unsecurified(_) => role_kind == RoleKind::Primary,
            Self::Securified(_) => true, // For securified we have all 5 variants
        }
    }

    pub(crate) fn update_with_entity_signed_for(
        &mut self,
        intent_with_signatures: EntitySignedFor,
    ) {
        match self {
            Self::Unsecurified(unsec) => {
                unsec.update_with_intent_with_signatures(intent_with_signatures)
            }
            Self::Securified(sec) => {
                sec.update_with_entity_signed_for(intent_with_signatures)
            }
        }
    }
    pub(crate) fn update_with_entity_not_signed_for(
        &mut self,
        not_signed: EntityNotSignedFor,
    ) {
        match self {
            Self::Unsecurified(unsec) => {
                unsec.update_with_entity_not_signed_for(not_signed)
            }
            Self::Securified(sec) => {
                sec.update_with_entity_not_signed_for(not_signed)
            }
        }
    }
}
impl From<(SecurityShieldApplicationWithTransactionIntents, IntentSetID)>
    for IntentSetInternalState
{
    fn from(
        (shield_application, intent_set_id): (
            SecurityShieldApplicationWithTransactionIntents,
            IntentSetID,
        ),
    ) -> Self {
        match shield_application {
            SecurityShieldApplicationWithTransactionIntents::ForSecurifiedEntity(sec) => {
                Self::Securified(SecurifiedIntentSetInternalState::from((sec, intent_set_id)))
            },
            SecurityShieldApplicationWithTransactionIntents::ForUnsecurifiedEntity(unsec) => {
                Self::Unsecurified(UnsecurifiedIntentSetInternalState::from((unsec, intent_set_id)))
            },
        }
    }
}
