#![cfg(test)]
#![allow(unused)]

use crate::prelude::*;

pub(crate) struct TestDerivationInteractors {
    pub(crate) poly: Arc<dyn PolyFactorKeyDerivationInteractor>,
    pub(crate) mono: Arc<dyn MonoFactorKeyDerivationInteractor>,
}

impl TestDerivationInteractors {
    pub(crate) fn new(
        poly: impl PolyFactorKeyDerivationInteractor + 'static,
        mono: impl MonoFactorKeyDerivationInteractor + 'static,
    ) -> Self {
        Self {
            poly: Arc::new(poly),
            mono: Arc::new(mono),
        }
    }

    pub(crate) fn with_secure_storage(
        secure_storage_client: SecureStorageClient,
    ) -> Self {
        let interactor = Arc::new(TestDerivationMonoAndPolyInteractor::new(
            false,
            secure_storage_client.clone(),
        ));
        Self {
            mono: interactor.clone(),
            poly: interactor.clone(),
        }
    }
}

impl TestDerivationInteractors {
    pub(crate) fn fail() -> Self {
        Self::new(
            TestDerivationMonoAndPolyInteractor::fail(),
            TestDerivationMonoAndPolyInteractor::fail(),
        )
    }
}
impl Default for TestDerivationInteractors {
    fn default() -> Self {
        Self::new(
            TestDerivationMonoAndPolyInteractor::default(),
            TestDerivationMonoAndPolyInteractor::default(),
        )
    }
}

impl KeysDerivationInteractors for TestDerivationInteractors {
    fn interactor_for(
        &self,
        kind: FactorSourceKind,
    ) -> KeyDerivationInteractor {
        match kind {
            FactorSourceKind::Device => {
                KeyDerivationInteractor::poly(self.poly.clone())
            }
            _ => KeyDerivationInteractor::mono(self.mono.clone()),
        }
    }
}
