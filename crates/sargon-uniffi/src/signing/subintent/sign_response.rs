use crate::prelude::*;
use sargon::FactorSourceIDFromHash as InternalFactorSourceIDFromHash;
use sargon::IndexMap;
use sargon::IndexSet;
use sargon::SignResponse as InternalSignResponse;
use sargon::SubintentHash as InternalSubintentHash;

type InternalSignResponseForSubintent =
    InternalSignResponse<InternalSubintentHash>;

/// The response of a batch signing request, either a PolyFactor or MonoFactor signing
/// request, matters not, because the goal is to have signed all transactions with
/// enough keys (derivation paths) needed for it to be valid when submitted to the
/// Radix network.
#[derive(Clone, PartialEq, Eq, uniffi::Record)]
pub struct SignResponseForSubintent {
    pub signatures:
        HashMap<FactorSourceIDFromHash, Vec<HdSignatureForSubintent>>,
}

impl SignResponseForSubintent {
    pub fn into_internal(&self) -> InternalSignResponseForSubintent {
        self.clone().into()
    }
}

impl From<InternalSignResponseForSubintent> for SignResponseForSubintent {
    fn from(value: InternalSignResponseForSubintent) -> Self {
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

impl From<SignResponseForSubintent> for InternalSignResponseForSubintent {
    fn from(value: SignResponseForSubintent) -> Self {
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

decl_conversion_tests_for!(SignResponseForSubintent);
