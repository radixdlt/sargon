use std::hash::Hasher;
use crate::prelude::*;

pub trait Signable: std::hash::Hash + PartialEq + Eq + Clone {
    type Payload: PartialEq + Eq + Clone + Debug + std::hash::Hash + Identifiable;

    fn get_payload(&self) -> Self::Payload;

    fn entities_requiring_signing(&self, profile: &Profile) -> Result<IndexSet<AccountOrPersona>>;

    fn get_id(&self) -> <Self::Payload as Identifiable>::ID {
        self.get_payload().id()
    }
}

////////////////////////////////////////////////////////////////////////
impl Signable for TransactionIntent {
    type Payload = CompiledTransactionIntent;

    fn get_payload(&self) -> Self::Payload {
        self.compile()
    }

    fn entities_requiring_signing(&self, profile: &Profile) -> Result<IndexSet<AccountOrPersona>> {
        let summary = self.manifest_summary()?;

        ExtractorOfEntitiesRequiringAuth::extract(
            profile,
            summary
        )
    }
}

impl std::hash::Hash for TransactionIntent {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.transaction_intent_hash().hash.0.to_string().hash(state)
    }
}

impl Identifiable for CompiledTransactionIntent {
    type ID = TransactionIntentHash;

    fn id(&self) -> Self::ID {
        self.decompile().transaction_intent_hash()
    }
}

impl From<CompiledTransactionIntent> for Hash {
    fn from(value: CompiledTransactionIntent) -> Self {
        value.decompile().transaction_intent_hash().hash
    }
}

impl Identifiable for TransactionIntent {
    type ID = TransactionIntentHash;

    fn id(&self) -> Self::ID {
        self.transaction_intent_hash()
    }
}