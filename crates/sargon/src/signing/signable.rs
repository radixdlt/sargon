use crate::prelude::*;

pub trait Signable: Identifiable {

    type Payload: SignablePayload + PartialEq + Eq + Clone + Debug + std::hash::Hash;

    type SignableID: SignableID;

    fn get_payload(&self) -> Self::Payload;

    fn get_id(&self) -> Self::SignableID;

    fn entities_requiring_signing(&self, profile: &Profile) -> Result<IndexSet<AccountOrPersona>>;
}

pub trait SignablePayload {
    type PayloadId: SignableID;

    fn get_payload_id(&self) -> Self::PayloadId;
}

pub trait SignableID: Into<Hash> + Clone + PartialEq + Eq + std::hash::Hash + Debug {}

////////////////////////////////////////////////////////////////////////
impl Signable for TransactionIntent {
    type Payload = CompiledTransactionIntent;
    type SignableID = TransactionIntentHash;

    fn get_payload(&self) -> Self::Payload {
        self.compile()
    }

    fn get_id(&self) -> Self::SignableID {
        self.transaction_intent_hash().clone()
    }

    fn entities_requiring_signing(&self, profile: &Profile) -> Result<IndexSet<AccountOrPersona>> {
        let summary = self.manifest_summary()?;

        ExtractorOfEntitiesRequiringAuth::extract(
            profile,
            summary
        )
    }
}

impl SignablePayload for CompiledTransactionIntent {
    type PayloadId = TransactionIntentHash;

    fn get_payload_id(&self) -> Self::PayloadId {
        self.decompile().transaction_intent_hash()
    }
}

impl SignableID for TransactionIntentHash {}

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