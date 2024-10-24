use crate::prelude::*;

pub trait Signable: Identifiable {

    type Payload: SignablePayload;

    type SignableID: SignableID;

    fn get_payload(&self) -> Self::Payload;

    fn get_id(&self) -> Self::SignableID;
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