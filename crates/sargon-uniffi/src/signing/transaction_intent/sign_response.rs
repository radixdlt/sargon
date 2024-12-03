use crate::prelude::*;
use sargon::FactorSourceIDFromHash as InternalFactorSourceIDFromHash;
use sargon::IndexMap;
use sargon::IndexSet;
use sargon::SignResponse as InternalSignResponse;
use sargon::TransactionIntentHash as InternalTransactionIntentHash;

type InternalSignResponseForTransactionIntent =
    InternalSignResponse<InternalTransactionIntentHash>;

/// The response of a batch signing request, either a PolyFactor or MonoFactor signing
/// request, matters not, because the goal is to have signed all transactions with
/// enough keys (derivation paths) needed for it to be valid when submitted to the
/// Radix network.
#[derive(Clone, PartialEq, Eq, uniffi::Record)]
pub struct SignResponseForTransactionIntent {
    pub per_factor_source: Vec<SignaturesPerFactorSourceOfTransactionIntentHash>,
}

impl SignResponseForTransactionIntent {
    pub fn into_internal(&self) -> InternalSignResponseForTransactionIntent {
        self.clone().into()
    }
}

impl From<InternalSignResponseForTransactionIntent>
    for SignResponseForTransactionIntent
{
    fn from(value: InternalSignResponseForTransactionIntent) -> Self {
        Self {
            per_factor_source: value.signatures
                .into_iter()
                .map(|(id, hd_signatures)| {
                    SignaturesPerFactorSourceOfTransactionIntentHash::new(
                        id.into(),
                        hd_signatures.into_iter().map(|s| s.into()).collect()
                    )
                })
                .collect()
        }
    }
}

impl From<SignResponseForTransactionIntent>
    for InternalSignResponseForTransactionIntent
{
    fn from(value: SignResponseForTransactionIntent) -> Self {
        Self {
            signatures: IndexMap::from_iter(value.per_factor_source.into_iter().map(
                |item| {
                    (
                        item.factor_source_id.into_internal(),
                        IndexSet::from_iter(
                            item.hd_signatures.into_iter().map(|s| s.into_internal()),
                        ),
                    )
                },
            )),
        }
    }
}

decl_conversion_tests_for!(SignResponseForTransactionIntent);
