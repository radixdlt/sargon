use crate::prelude::*;
use sargon::OsFactorSourceAdder;

#[uniffi::export]
impl SargonOS {
    async fn is_factor_source_already_in_use(
        &self,
        factor_source_id: FactorSourceID,
    ) -> Result<bool> {
        self.wrapped
            .is_factor_source_already_in_use(factor_source_id.into_internal())
            .await
            .into_result()
    }

    async fn add_new_mnemonic_factor_source(
        &self,
        factor_source_kind: FactorSourceKind,
        mnemonic_with_passphrase: MnemonicWithPassphrase,
        name: String,
    ) -> Result<FactorSourceID> {
        self.wrapped
            .add_new_mnemonic_factor_source(
                factor_source_kind.into_internal(),
                mnemonic_with_passphrase.into_internal(),
                name,
            )
            .await
            .into_result()
    }
}
