use crate::prelude::*;

/// A collection of interactors that the host is providing during boot.
/// Such interactors are used to drive ui from within sargon os.
pub struct Interactors {
    /// Interactors related to factor sources.
    pub use_factor_sources_interactor: Arc<dyn UseFactorSourcesInteractor>,
}

impl Interactors {
    pub fn new(
        use_factor_sources_interactor: Arc<dyn UseFactorSourcesInteractor>,
    ) -> Self {
        Self {
            use_factor_sources_interactor,
        }
    }
}

#[cfg(test)]
impl Interactors {
    pub fn new_with_derivation_interactor(
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
            );

        Self::new(Arc::new(use_factor_sources_interactors))
    }

    pub fn new_from_clients(clients: &Clients) -> Self {
        Self::new_with_derivation_interactor(Arc::new(
            TestDerivationInteractor::new(
                false,
                Arc::new(clients.secure_storage.clone()),
            ),
        ))
    }
}
