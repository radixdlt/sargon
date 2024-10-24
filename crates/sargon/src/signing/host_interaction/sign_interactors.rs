use crate::prelude::*;

/// A collection of "interactors" which can sign transactions.
pub trait SignInteractors<SP: SignablePayload> {
    fn interactor_for(&self, kind: FactorSourceKind) -> SignInteractor<SP>;
}
