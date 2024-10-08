use crate::prelude::*;
use sargon::RequestedQuantity as InternalRequestedQuantity; 

/// A requested (by Dapp) quantity, e.g. "I want AT LEAST 3 account addresses" or
/// "I want EXACTLY 2 email addresses".
#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    Hash,
    
    uniffi::Record,
)]
pub struct RequestedQuantity {
    pub quantifier: RequestedNumberQuantifier,
    pub quantity: u16,
}

impl From<InternalRequestedQuantity> for RequestedQuantity {
    fn from(value: InternalRequestedQuantity) -> Self {
        Self {
            quantifier: value.quantifier.into(),
            quantity: value.quantity,
        }
    }
}

impl Into<InternalRequestedQuantity> for RequestedQuantity {
    fn into(self) -> InternalRequestedQuantity {
        InternalRequestedQuantity {
            quantifier: self.quantifier.into(),
            quantity: self.quantity,
        }
    }
}