use crate::prelude::*;

#[derive(PartialEq, Eq, Clone)]
pub struct Domain(String);

impl Domain {
    pub fn new(raw_domain: String) -> Self {
        Self(raw_domain)
    }
}

impl Domain {
    pub fn to_non_fungible_id(&self) -> Result<NonFungibleLocalId> {
        domain_to_non_fungible_id(&self.0, true)
    }
}