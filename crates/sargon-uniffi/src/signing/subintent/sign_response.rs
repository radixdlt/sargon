use crate::prelude::*;
use sargon::SubintentHash as InternalSubintentHash;
use sargon::FactorSourceIDFromHash as InternalFactorSourceIDFromHash;
use sargon::IndexSet;
use sargon::IndexMap;

type InternalSignResponse = sargon::SignResponse<InternalSubintentHash>;

/// The response of a batch signing request, either a PolyFactor or MonoFactor signing
/// request, matters not, because the goal is to have signed all transactions with
/// enough keys (derivation paths) needed for it to be valid when submitted to the
/// Radix network.
#[derive(Clone, PartialEq, Eq, uniffi::Record)]
pub struct SignResponseForSubintent {
    pub signatures: HashMap<FactorSourceIDFromHash, Vec<HdSignatureForSubintent>>,
}

impl SignResponseForSubintent {

    pub fn into_internal(&self) -> InternalSignResponse {
        self.clone().into()
    }

}

impl From<InternalSignResponse> for SignResponseForSubintent {
    fn from(value: InternalSignResponse) -> Self {
        Self {
            signatures: value
                .signatures
                .into_iter()
                .map(|(id, signatures)|  {
                    (id.into(), signatures.into_iter().map(|s| s.into()).collect())
                })
                .collect(),
        }
    }
}

impl From<SignResponseForSubintent> for InternalSignResponse {
    fn from(value: SignResponseForSubintent) -> Self {
        Self {
            signatures: IndexMap::from_iter(
                value
                    .signatures
                    .into_iter()
                    .map(|(id, signatures)| {
                        (
                            id.into_internal(),
                            IndexSet::from_iter(signatures.into_iter().map(|s| s.into_internal()))
                        )
                    })
            ),
        }
    }
}