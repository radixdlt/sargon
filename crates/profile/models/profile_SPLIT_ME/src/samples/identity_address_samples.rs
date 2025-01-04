use crate::prelude::*;

pub trait HasManySampleValues: Sized {
    #[allow(dead_code)]
    fn sample_all() -> Vec<Self>;
}

pub trait HasIndexedSampleValues: Sized {
    #[allow(dead_code)]
    fn sample_at(index: usize) -> Self;
}

impl HasIndexedSampleValues for IdentityAddress {
    fn sample_at(index: usize) -> Self {
        Persona::sample_at(index).address
    }
}

impl HasManySampleValues for IdentityAddress {
    fn sample_all() -> Vec<Self> {
        Persona::sample_all().iter().map(|a| a.address).collect()
    }
}
