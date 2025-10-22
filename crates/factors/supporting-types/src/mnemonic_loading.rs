use crate::prelude::*;

#[async_trait::async_trait]
pub trait MnemonicLoading: std::fmt::Debug + Send + Sync {
    async fn load_mnemonic(
        &self,
        id: FactorSourceIDFromHash,
    ) -> Result<MnemonicWithPassphrase>;
}

#[derive(Debug, Default, Clone)]
pub struct FailingMnemonicLoader;

#[async_trait::async_trait]
impl MnemonicLoading for FailingMnemonicLoader {
    async fn load_mnemonic(
        &self,
        _id: FactorSourceIDFromHash,
    ) -> Result<MnemonicWithPassphrase> {
        Err(CommonError::Unknown {
            error_message: "Failed loading mnemonic".to_string(),
        })
    }
}
