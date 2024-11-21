use crate::prelude::*;

/// An interactor which can derive keys - either in multiple factor sources in
/// one go, or a single one.
pub enum KeyDerivationInteractor {
    /// Many factor sources used to derive keys.
    PolyFactor(Arc<dyn PolyFactorKeyDerivationInteractor>),

    /// A single factor source used to derive keys.
    MonoFactor(Arc<dyn MonoFactorKeyDerivationInteractor>),
}

impl KeyDerivationInteractor {
    #[allow(unused)]
    pub(crate) fn poly(
        interactor: Arc<dyn PolyFactorKeyDerivationInteractor>,
    ) -> Self {
        Self::PolyFactor(interactor)
    }

    #[allow(unused)]
    pub(crate) fn mono(
        interactor: Arc<dyn MonoFactorKeyDerivationInteractor>,
    ) -> Self {
        Self::MonoFactor(interactor)
    }
}
