use crate::prelude::*;

/// A collection of "interactors" which can sign transactions.
pub trait SignInteractors<S: Signable> {
    fn interactor_for(&self, kind: FactorSourceKind) -> SignInteractor<S>;
}
