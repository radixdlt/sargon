use crate::prelude::*;

impl IdentityAddress {
    pub fn sample_at(index: usize) -> Self {
        Persona::sample_at(index).address
    }

    pub fn sample_all() -> Vec<Self> {
        Persona::sample_all().iter().map(|a| a.address).collect()
    }
}
