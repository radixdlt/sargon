use crate::prelude::*;

/// A collection of interactors that the host is providing during boot.
/// Such interactors are used to drive ui from within sargon os.
pub struct Interactors {
    /// Interactors related to factor sources.
    pub use_factor_sources_interactor: Arc<dyn UseFactorSourcesInteractor>,

    /// Interactor that asks the user to authorize
    pub authorization_interactor: Arc<dyn AuthorizationInteractor>,

    /// Interactor that asks the user perform a spot check
    pub spot_check_interactor: Arc<dyn SpotCheckInteractor>,
}

impl Interactors {
    pub fn new(
        use_factor_sources_interactor: Arc<dyn UseFactorSourcesInteractor>,
        authorization_interactor: Arc<dyn AuthorizationInteractor>,
        spot_check_interactor: Arc<dyn SpotCheckInteractor>,
    ) -> Self {
        Self {
            use_factor_sources_interactor,
            authorization_interactor,
            spot_check_interactor,
        }
    }
}
