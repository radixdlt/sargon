use crate::prelude::*;
use sargon::FactorSourceIDFromHash as InternalFactorSourceIDFromHash;
use sargon::IndexMap;
use sargon::IndexSet;
use sargon::TransactionIntentHash as InternalTransactionIntentHash;

type InternalSignResponse = sargon::SignResponse<InternalTransactionIntentHash>;

/// The response of a batch signing request, either a PolyFactor or MonoFactor signing
/// request, matters not, because the goal is to have signed all transactions with
/// enough keys (derivation paths) needed for it to be valid when submitted to the
/// Radix network.
#[derive(Clone, PartialEq, Eq, uniffi::Record)]
pub struct SignResponseForTransactionIntent {
    pub signatures:
        HashMap<FactorSourceIDFromHash, Vec<HdSignatureForTransactionIntent>>,
}

impl SignResponseForTransactionIntent {
    pub fn into_internal(&self) -> InternalSignResponse {
        self.clone().into()
    }
}

impl From<InternalSignResponse> for SignResponseForTransactionIntent {
    fn from(value: InternalSignResponse) -> Self {
        Self {
            signatures: value
                .signatures
                .into_iter()
                .map(|(id, signatures)| {
                    (
                        id.into(),
                        signatures.into_iter().map(|s| s.into()).collect(),
                    )
                })
                .collect(),
        }
    }
}

impl From<SignResponseForTransactionIntent> for InternalSignResponse {
    fn from(value: SignResponseForTransactionIntent) -> Self {
        Self {
            signatures: IndexMap::from_iter(value.signatures.into_iter().map(
                |(id, signatures)| {
                    (
                        id.into_internal(),
                        IndexSet::from_iter(
                            signatures.into_iter().map(|s| s.into_internal()),
                        ),
                    )
                },
            )),
        }
    }
}
