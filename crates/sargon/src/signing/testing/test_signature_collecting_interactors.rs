#![cfg(test)]
#![allow(unused)]

use crate::prelude::*;

pub(crate) struct TestSignatureCollectingInteractors {
    pub(crate) simulated_user: SimulatedUser,
}

impl TestSignatureCollectingInteractors {
    pub(crate) fn new(simulated_user: SimulatedUser) -> Self {
        Self { simulated_user }
    }
}

impl SignInteractors<CompiledTransactionIntent> for TestSignatureCollectingInteractors {
    fn interactor_for(&self, kind: FactorSourceKind) -> SignInteractor<CompiledTransactionIntent> {
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
