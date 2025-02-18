use crate::prelude::*;

#[derive(Clone, PartialEq, Eq, StdHash, derive_more::Debug)]
pub(crate) struct EntitySigningOutcome<Outcome> {
    pub context: EntitySigningContext,
    pub intent: TransactionIntent,
    pub entity: AccountOrPersona,
    outcome: Outcome,
}

impl<Outcome> EntitySigningOutcome<Outcome> {
    pub fn role_kind(&self) -> RoleKind {
        self.context.role_kind
    }
    pub fn intent_set_id(&self) -> IntentSetID {
        self.context.intent_set_id
    }
    pub(crate) fn new(
        context: EntitySigningContext,
        intent: TransactionIntent,
        entity: AccountOrPersona,
        outcome: Outcome,
    ) -> Self {
        Self {
            context,
            intent,
            entity,
            outcome,
        }
    }
}

pub type EntityNotSignedFor = EntitySigningOutcome<IndexSet<NeglectedFactor>>;

pub type EntitySignedFor =
    EntitySigningOutcome<IndexSet<SignatureWithPublicKey>>;

impl EntitySignedFor {
    pub fn intent_signatures(&self) -> IndexSet<IntentSignature> {
        self.outcome
            .clone()
            .into_iter()
            .map(IntentSignature::from)
            .collect::<IndexSet<_>>()
    }
}

impl EntityNotSignedFor {
    pub fn neglected_factor_sources(&self) -> IndexSet<NeglectedFactor> {
        self.outcome.clone()
    }
}
