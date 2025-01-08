use crate::prelude::*;

pub trait HasIndexedAccountSampleValues: Sized {
    #[allow(dead_code)]
    fn sample_at(index: usize) -> Self;
}

pub trait HasManyAccountSampleValues: Sized {
    #[allow(dead_code)]
    fn sample_all() -> Vec<Self>;
}

impl HasIndexedAccountSampleValues for AccountAddress {
    fn sample_at(index: usize) -> Self {
        Account::sample_at(index).address
    }
}

impl HasManyAccountSampleValues for AccountAddress {
    fn sample_all() -> Vec<Self> {
        Account::sample_all().iter().map(|a| a.address).collect()
    }
}
