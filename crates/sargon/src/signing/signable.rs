use crate::prelude::*;

pub trait Signable: Identifiable {

    type Payload: SignablePayload;

    fn get_payload(&self) -> Self::Payload;
}

pub trait SignablePayload {
    fn get_payload_identifier(&self) -> impl Into<Hash>;
}

////////////////////////////////////////////////////////////////////////
impl Signable for TransactionIntent {
    type Payload = CompiledTransactionIntent;

    fn get_payload(&self) -> Self::Payload {
        self.compile()
    }

}

impl SignablePayload for CompiledTransactionIntent {
    fn get_payload_identifier(&self) -> impl Into<Hash> {
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