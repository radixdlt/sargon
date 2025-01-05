use crate::prelude::*;

pub trait HasIndexedPersonaSampleValues: Sized {
    #[allow(dead_code)]
    fn sample_at(index: usize) -> Self;
}

impl HasIndexedPersonaSampleValues for IdentityAddress {
    fn sample_at(index: usize) -> Self {
        Persona::sample_at(index).address
    }
}

pub trait HasManyPersonaSampleValues: Sized {
    #[allow(dead_code)]
    fn sample_all() -> Vec<Self>;
}

impl HasManyPersonaSampleValues for IdentityAddress {
    fn sample_all() -> Vec<Self> {
        Persona::sample_all().iter().map(|a| a.address).collect()
    }
}
