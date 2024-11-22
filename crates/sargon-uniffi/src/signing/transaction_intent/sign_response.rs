use crate::prelude::*;

// /// The response of a batch signing request, either a PolyFactor or MonoFactor signing
// /// request, matters not, because the goal is to have signed all transactions with
// /// enough keys (derivation paths) needed for it to be valid when submitted to the
// /// Radix network.
// #[derive(Clone, PartialEq, Eq, derive_more::Debug)]
// #[debug("SignResponse {{ signatures: {:#?} }}", signatures.values().map(|f| format!("{:#?}", f)).join(", "))]
// pub struct SignResponse<ID: SignableID> {
//     pub signatures: IndexMap<sargon::FactorSourceIDFromHash, IndexSet<HDSignature<ID>>>,
// }