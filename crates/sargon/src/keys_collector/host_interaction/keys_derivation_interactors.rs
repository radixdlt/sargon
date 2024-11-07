use crate::prelude::*;

/// A collection of "interactors" which can derive keys.
pub trait KeysDerivationInteractors: Sync + Send {
    fn interactor_for(&self, kind: FactorSourceKind)
        -> KeyDerivationInteractor;
}
