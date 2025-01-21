use crate::prelude::*;
use sargon::DerivePublicKeysSource as InternalDerivePublicKeysSource;
use sargon::OsDerivePublicKeys;

#[uniffi::export]
impl SargonOS {
    pub async fn derive_public_keys(
        &self,
        derivation_paths: Vec<DerivationPath>,
        source: DerivePublicKeysSource,
    ) -> Result<Vec<HierarchicalDeterministicPublicKey>> {
        self.wrapped
            .derive_public_keys(
                derivation_paths.into_internal(),
                source.into_internal(),
            )
            .await
            .into_iter_result()
    }
}

#[derive(Clone, PartialEq, InternalConversion, uniffi::Enum)]
pub enum DerivePublicKeysSource {
    MnemonicWithPassphrase(MnemonicWithPassphrase),

    FactorSource(FactorSourceIDFromHash),
}
