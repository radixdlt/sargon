use crate::prelude::*;

/// An interactor responsible for communicating with the user on host, to perform a spot check
/// on a factor source.
#[async_trait::async_trait]
pub trait SpotCheckInteractor: Send + Sync {
    async fn spot_check(
        &self,
        factor_source_id: FactorSourceIDFromHash,
    ) -> Result<SpotCheckResponse>;
}

#[derive(Clone, Debug, PartialEq, Eq, std::hash::Hash)]
pub enum SpotCheckResponse {
    /// The user retrieved the id of a Ledger device.
    /// Used for the identification of `LedgerHardwareWalletFactorSource`.
    Ledger { id: Exactly32Bytes },

    /// The user retrieved a `MnemonicWithPassphrase`.
    /// Used for the identification of any other `FactorSource`.
    MnemonicWithPassphrase { value: MnemonicWithPassphrase },
}
