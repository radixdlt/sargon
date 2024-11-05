#![allow(unused)]

use crate::prelude::*;

pub struct TestSignatureCollectingInteractors {
    pub simulated_user: SimulatedUser,
}

impl TestSignatureCollectingInteractors {
    pub fn new(simulated_user: SimulatedUser) -> Self {
        Self { simulated_user }
    }
}

impl SignInteractors<TransactionIntent> for TestSignatureCollectingInteractors {
    fn interactor_for(
        &self,
        kind: FactorSourceKind,
    ) -> SignInteractor<TransactionIntent> {
        match kind {
            FactorSourceKind::Device => SignInteractor::poly(Arc::new(
                TestSigningParallelInteractor::new(self.simulated_user.clone()),
            )),
            _ => SignInteractor::mono(Arc::new(
                TestSigningSerialInteractor::new(self.simulated_user.clone()),
            )),
        }
    }
}
