use crate::prelude::*;

impl AccountAddress {
    pub fn sample_at(index: usize) -> Self {
        Account::sample_at(index).address
    }

    pub fn sample_all() -> Vec<Self> {
        Account::sample_all().iter().map(|a| a.address).collect()
    }
}
