use crate::prelude::*;

pub trait InteractorsCtors {
    fn new_with_derivation_interactor(
        keys_derivation_interactor: Arc<dyn KeyDerivationInteractor>,
    ) -> Interactors;

    fn new_from_clients(clients: &Clients) -> Interactors {
        Self::new_with_derivation_interactor(Arc::new(
            TestDerivationInteractor::new(
                false,
                Arc::new(clients.secure_storage.clone()),
            ),
        ))
    }
}

impl InteractorsCtors for Interactors {
    fn new_with_derivation_interactor(
        keys_derivation_interactor: Arc<dyn KeyDerivationInteractor>,
    ) -> Self {
        let use_factor_sources_interactors =
            TestUseFactorSourcesInteractors::new(
                Arc::new(TestSignInteractor::<TransactionIntent>::new(
                    SimulatedUser::prudent_no_fail(),
                )),
                Arc::new(TestSignInteractor::<Subintent>::new(
                    SimulatedUser::prudent_no_fail(),
                )),
                keys_derivation_interactor,
                Arc::new(TestSignInteractor::<AuthIntent>::new(
                    SimulatedUser::prudent_no_fail(),
                )),
            );

        Self::new(Arc::new(use_factor_sources_interactors))
    }
}
