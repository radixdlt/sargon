use std::hash::Hasher;
use crate::prelude::*;

pub trait Signable: std::hash::Hash + PartialEq + Eq + Clone {
    type ID: SignableID;

    type Payload: PartialEq + Eq + Clone + Debug + std::hash::Hash + Into<Self::ID>;

    fn get_payload(&self) -> Self::Payload;

    fn entities_requiring_signing(&self, profile: &Profile) -> Result<IndexSet<AccountOrPersona>>;

    fn get_id(&self) -> Self::ID {
        self.get_payload().into()
    }
}

pub trait SignableID: Eq + StdHash + Clone + Debug + Into<Hash> {}

////////////////////////////////////////////////////////////////////////
impl Signable for TransactionIntent {
    type ID = TransactionIntentHash;

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

impl SignableID for TransactionIntentHash {}

impl std::hash::Hash for TransactionIntent {
    fn hash<H: Hasher>(&self, state: &mut H) {
        state.write(self.transaction_intent_hash().hash.as_ref())
    }
}

impl Into<TransactionIntentHash> for CompiledTransactionIntent {
    fn into(self) -> TransactionIntentHash {
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