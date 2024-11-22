use crate::prelude::*;

/// A collection of "interactors", a bridge between SargonOS and hosts for "interactions" - performing
/// actions/operations that require the host to display UI and async await user input. Such
/// as the interactors used for SignaturesCollector and KeysCollector when using FactorSource.
pub struct Interactors {
    /// Interactor for key derivation, used by the KeysCollector, which in turn is used by
    /// FactorInstancesProvider for all operations involving new keys, such as account creation
    /// and securifying entities (mapping SecurityStructureOfFactorSource -> SecurityStructureOfFactorInstances).
    pub key_derivation: Arc<dyn KeysDerivationInteractors>,
}

impl Interactors {
    pub fn new(key_derivation: Arc<dyn KeysDerivationInteractors>) -> Self {
        Self { key_derivation }
    }
}
