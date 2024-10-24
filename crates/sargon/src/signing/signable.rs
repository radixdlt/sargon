use crate::prelude::*;

pub trait Signable: Identifiable {

    type Payload: SignablePayload;

    fn get_payload(&self) -> Self::Payload;
}

pub trait SignablePayload {
    type PayloadId: SignablePayloadID;

    fn get_payload_id(&self) -> Self::PayloadId;
}

pub trait SignablePayloadID: Into<Hash> + Clone + PartialEq + Eq + std::hash::Hash {}

////////////////////////////////////////////////////////////////////////
impl Signable for TransactionIntent {
    type Payload = CompiledTransactionIntent;

    fn get_payload(&self) -> Self::Payload {
        self.compile()
    }

}

impl SignablePayload for CompiledTransactionIntent {
    type PayloadId = TransactionIntentHash;

    fn get_payload_id(&self) -> Self::PayloadId {
        self.decompile().transaction_intent_hash()
    }
}

impl SignablePayloadID for TransactionIntentHash {}

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