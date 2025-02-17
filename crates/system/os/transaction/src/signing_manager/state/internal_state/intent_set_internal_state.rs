use crate::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq)]
#[allow(clippy::large_enum_variant)]
pub(crate) enum IntentSetInternalState {
    Unsecurified(UnsecurifiedIntentSetInternalState),
    Securified(SecurifiedIntentSetInternalState),
}
impl IntentSetInternalState {
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

    pub(crate) fn update_with_intent_with_signatures(
        &mut self,
        intent_with_signatures: EntitySignedFor,
    ) {
        match self {
            Self::Unsecurified(unsec) => {
                unsec.update_with_intent_with_signatures(intent_with_signatures)
            }
            Self::Securified(sec) => {
                sec.update_with_intent_with_signatures(intent_with_signatures)
            }
        }
    }
}
impl From<SecurityShieldApplicationWithTransactionIntents>
    for IntentSetInternalState
{
    fn from(
        shield_application: SecurityShieldApplicationWithTransactionIntents,
    ) -> Self {
        match shield_application {
            SecurityShieldApplicationWithTransactionIntents::ForSecurifiedEntity(sec) => {
                Self::Securified(SecurifiedIntentSetInternalState::from(sec))
            },
            SecurityShieldApplicationWithTransactionIntents::ForUnsecurifiedEntity(unsec) => {
                Self::Unsecurified(UnsecurifiedIntentSetInternalState::from(unsec))
            },
        }
    }
}
