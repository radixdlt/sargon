use crate::prelude::*;
use crate::types::samples::{HasIndexedSampleValues, HasManySampleValues};

impl HasIndexedSampleValues for AccountAddress {
    fn sample_at(index: usize) -> Self {
        Account::sample_at(index).address
    }
}

impl HasManySampleValues for AccountAddress {
    fn sample_all() -> Vec<Self> {
        Account::sample_all().iter().map(|a| a.address).collect()
    }
}
