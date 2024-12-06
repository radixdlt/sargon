use crate::prelude::*;

pub(super) struct SignaturesCollectorDependencies<S: Signable> {
    /// If `true` we stop collecting signatures as soon as all transactions are
    /// valid. This is typically always set to `true`, but can be useful for
    /// tests to set to `false` to see how the system behaves.
    pub(super) finish_early_strategy: SigningFinishEarlyStrategy,

    /// The interactor that is used to sign with factor sources.
    pub(super) interactor: Arc<dyn SignInteractor<S>>,
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

impl<S: Signable> SignaturesCollectorDependencies<S> {
    pub(crate) fn new(
        finish_early_strategy: SigningFinishEarlyStrategy,
        interactor: Arc<dyn SignInteractor<S>>,
        factors_of_kind: IndexSet<FactorSourcesOfKind>,
    ) -> Self {
        Self {
            finish_early_strategy,
            interactor,
            factors_of_kind,
        }
    }
}
