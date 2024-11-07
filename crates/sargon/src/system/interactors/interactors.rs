use crate::prelude::*;

pub struct Interactors {
    pub key_derivation: Arc<dyn KeysDerivationInteractors>,
}

impl Interactors {
    pub fn new(key_derivation: Arc<dyn KeysDerivationInteractors>) -> Self {
        Self { key_derivation }
    }
}
