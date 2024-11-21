use crate::prelude::*;

pub(crate) struct KeysCollectorDependencies {
    /// A collection of "interactors" used to sign with factor sources.
    pub(super) interactors: Arc<dyn KeysDerivationInteractors>,

    /// Factor sources grouped by kind, sorted according to "friction order",
    /// that is, we want to control which FactorSourceKind users sign with
    /// first, second etc, e.g. typically we prompt user to sign with Ledgers
    /// first, and if a user might lack access to that Ledger device, then it is
    /// best to "fail fast", otherwise we might waste the users time, if she has
    /// e.g. answered security questions and then is asked to use a Ledger
    /// she might not have handy at the moment - or might not be in front of a
    /// computer and thus unable to make a connection between the Radix Wallet
    /// and a Ledger device.
    pub(super) factors_of_kind: IndexSet<FactorSourcesOfKind>,
}

impl KeysCollectorDependencies {
    pub(crate) fn new(
        interactors: Arc<dyn KeysDerivationInteractors>,
        factors_of_kind: IndexSet<FactorSourcesOfKind>,
    ) -> Self {
        Self {
            interactors,
            factors_of_kind,
        }
    }
}
