use crate::prelude::*;

/// A compilation of multiple traits that the host should abide by
/// in order for sargon os to interact with it.
///
/// Such host should be able to:
/// - sign transactions with `SignInteractor<TransactionIntent>` which is used by `SignaturesCollector`
/// - sign subintents with `SignInteractor<Subintent>` which is used by `SignaturesCollector`
/// - derive keys with `KeyDerivationInteractor` which is used by `KeysCollector`
pub trait HostInteractor:
    SignInteractor<TransactionIntent>
    + SignInteractor<Subintent>
    + KeyDerivationInteractor
{
}
