use crate::prelude::*;

/// A compilation of interactors that use factor sources.
///
/// The host should be able to:
/// - sign transactions with `SignInteractor<TransactionIntent>` which is used by `SignaturesCollector`
/// - sign subintents with `SignInteractor<Subintent>` which is used by `SignaturesCollector`
/// - derive keys with `KeyDerivationInteractor` which is used by `KeysCollector`
pub trait UseFactorSourcesInteractor:
    SignInteractor<TransactionIntent>
    + SignInteractor<Subintent>
    + KeyDerivationInteractor
    + AuthenticationSigningInteractor
{
}
