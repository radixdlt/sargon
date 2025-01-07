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
