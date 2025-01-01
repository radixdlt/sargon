pub trait TestDerivationInteractorFromSecureStorageClient: Sized {
    fn new(
        always_fail: bool,
        secure_storage_client: Arc<SecureStorageClient>,
    ) -> Self;
}

#[async_trait::async_trait]
impl MnemonicLoading for SecureStorageClient {
    async fn load_mnemonic(
        &self,
        id: FactorSourceIDFromHash,
    ) -> Result<MnemonicWithPassphrase> {
        self.load_mnemonic_with_passphrase(id).await
    }
}

impl TestDerivationInteractorFromSecureStorageClient
    for TestDerivationInteractor
{
    fn new(
        always_fail: bool,
        secure_storage_client: Arc<SecureStorageClient>,
    ) -> Self {
        Self::with_mnemonic_loading(
            always_fail,
            secure_storage_client as Arc<dyn MnemonicLoading>,
        )
    }
}
