use crate::prelude::*;

pub trait TestDerivationInteractorFromSecureStorageClient: Sized {
    fn new(
        always_fail: bool,
        secure_storage_client: Arc<SecureStorageClient>,
    ) -> Self;
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
