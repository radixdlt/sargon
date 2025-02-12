#![allow(unused)]

use crate::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum SigningUserInput {
    Sign,
    Skip,
    Reject,
}

pub type SpyOnRequest<ID> =
    Arc<dyn Fn(FactorSourceKind, IndexSet<InvalidTransactionIfNeglected<ID>>)>;

#[derive(Clone, derive_more::Debug)]
#[debug("SimulatedUser(mode: {mode:?}, failures: {failures:?})")]
pub struct SimulatedUser<S: Signable> {
    spy_on_request: SpyOnRequest<S::ID>,
    mode: SimulatedUserMode,
    /// `None` means never failures
    failures: Option<SimulatedFailures>,
}

impl<S: Signable> SimulatedUser<S> {
    pub fn with_spy(
        spy_on_request: impl Fn(FactorSourceKind, IndexSet<InvalidTransactionIfNeglected<S::ID>>)
            + 'static,
        mode: SimulatedUserMode,
        failures: impl Into<Option<SimulatedFailures>>,
    ) -> Self {
        Self {
            spy_on_request: Arc::new(spy_on_request),
            mode,
            failures: failures.into(),
        }
    }

    pub fn new(
        mode: SimulatedUserMode,
        failures: impl Into<Option<SimulatedFailures>>,
    ) -> Self {
        Self::with_spy(|_, _| {}, mode, failures)
    }
}

#[derive(Debug, Clone, Default)]
pub struct SimulatedFailures {
    /// Set of FactorSources which should always fail.
    simulated_failures: IndexSet<FactorSourceIDFromHash>,
}

impl SimulatedFailures {
    pub fn with_details(
        simulated_failures: IndexSet<FactorSourceIDFromHash>,
    ) -> Self {
        Self { simulated_failures }
    }

    pub fn with_simulated_failures(
        failures: impl IntoIterator<Item = FactorSourceIDFromHash>,
    ) -> Self {
        Self::with_details(IndexSet::from_iter(failures))
    }

    /// If needed, simulates failure for ALL factor sources or NONE.
    pub(crate) fn simulate_failure_if_needed(
        &self,
        factor_source_ids: IndexSet<FactorSourceIDFromHash>,
    ) -> bool {
        factor_source_ids
            .into_iter()
            .all(|id| self.simulated_failures.contains(&id))
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum SimulatedUserMode {
    /// Emulation of a "prudent" user, that signs with all factors sources, i.e.
    /// she never ever "skips" a factor source
    Prudent,

    /// Emulation of a "lazy" user, that skips signing with as many factor
    /// sources as possible.
    Lazy(Laziness),

    /// Emulation of a user, that skips specific factor sources.
    Skipping(IndexSet<FactorSourceIDFromHash>),

    /// Emulation of a user that dismisses (rejects) the signing process all-together.
    Rejecting,
}

impl SimulatedUserMode {
    pub(crate) fn lazy_always_skip() -> Self {
        Self::Lazy(Laziness::AlwaysSkip)
    }

    /// Skips only if `invalid_tx_if_skipped` is empty
    pub(crate) fn lazy_sign_minimum() -> Self {
        Self::Lazy(Laziness::SignMinimum)
    }

    pub(crate) fn skipping_specific(
        factor_sources: IndexSet<FactorSourceIDFromHash>,
    ) -> Self {
        Self::Skipping(factor_sources)
    }
}

impl<S: Signable> SimulatedUser<S> {
    pub fn prudent_no_fail() -> Self {
        Self::new(SimulatedUserMode::Prudent, None)
    }

    pub fn rejecting() -> Self {
        Self::new(SimulatedUserMode::Rejecting, None)
    }

    pub fn prudent_with_failures(
        simulated_failures: SimulatedFailures,
    ) -> Self {
        Self::new(SimulatedUserMode::Prudent, simulated_failures)
    }

    pub fn skipping_specific(
        factor_sources: IndexSet<FactorSourceIDFromHash>,
    ) -> Self {
        Self::new(SimulatedUserMode::skipping_specific(factor_sources), None)
    }

    pub fn lazy_always_skip_no_fail() -> Self {
        Self::new(SimulatedUserMode::lazy_always_skip(), None)
    }

    /// Skips only if `invalid_tx_if_skipped` is empty
    /// (or if simulated failure for that factor source)
    pub fn lazy_sign_minimum(
        simulated_failures: impl IntoIterator<Item = FactorSourceIDFromHash>,
    ) -> Self {
        Self::new(
            SimulatedUserMode::lazy_sign_minimum(),
            SimulatedFailures::with_simulated_failures(simulated_failures),
        )
    }
}

unsafe impl<S: Signable> Sync for SimulatedUser<S> {}
unsafe impl<S: Signable> Send for SimulatedUser<S> {}

/// A very lazy user that defers all boring work such as signing stuff for as long
/// as possible. Ironically, this sometimes leads to user signing more than she
/// actually needs. For example, if the user has a Securified Account with threshold
/// and override factors, she actually needs to sign with a single override
/// factor. But since user is so lazy, she defers signing with that override
/// factor if prompted for it first.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Laziness {
    SignMinimum,
    AlwaysSkip,
}

impl<S: Signable> SimulatedUser<S> {
    pub(crate) fn spy_on_request_before_handled(
        &self,
        factor_source_kind: FactorSourceKind,
        invalid_tx_if_skipped: IndexSet<InvalidTransactionIfNeglected<S::ID>>,
    ) {
        (self.spy_on_request)(
            factor_source_kind,
            invalid_tx_if_skipped.clone(),
        );
    }

    pub(crate) fn sign_or_skip(
        &self,
        invalid_tx_if_skipped: impl IntoIterator<
            Item = InvalidTransactionIfNeglected<S::ID>,
        >,
    ) -> SigningUserInput {
        let invalid_tx_if_skipped = invalid_tx_if_skipped
            .into_iter()
            .collect::<std::collections::HashSet<_>>();

        if self.be_prudent(|| !invalid_tx_if_skipped.is_empty()) {
            SigningUserInput::Sign
        } else if self.mode == SimulatedUserMode::Rejecting {
            SigningUserInput::Reject
        } else {
            SigningUserInput::Skip
        }
    }

    pub(crate) fn simulate_failure_if_needed(
        &self,
        factor_source_ids: IndexSet<FactorSourceIDFromHash>,
    ) -> bool {
        if let Some(failures) = &self.failures {
            failures.simulate_failure_if_needed(factor_source_ids)
        } else {
            false
        }
    }

    pub(crate) fn simulate_skip_if_needed(
        &self,
        factor_source_ids: IndexSet<FactorSourceIDFromHash>,
    ) -> bool {
        match &self.mode {
            SimulatedUserMode::Skipping(skipping_factor_source_ids) => {
                factor_source_ids
                    .iter()
                    .all(|id| skipping_factor_source_ids.contains(id))
            }
            _ => false,
        }
    }

    fn be_prudent<F>(&self, is_prudent: F) -> bool
    where
        F: Fn() -> bool,
    {
        match &self.mode {
            SimulatedUserMode::Prudent => true,
            SimulatedUserMode::Lazy(laziness) => match laziness {
                Laziness::AlwaysSkip => false,
                Laziness::SignMinimum => is_prudent(),
            },
            SimulatedUserMode::Rejecting => false,
            SimulatedUserMode::Skipping(_) => false,
        }
    }
}
