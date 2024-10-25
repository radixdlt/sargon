use crate::prelude::*;

/// An interactor which can sign transactions - either in poly or mono.
pub enum SignInteractor<S: Signable> {
    PolyFactor(Arc<dyn PolyFactorSignInteractor<S>>),
    MonoFactor(Arc<dyn MonoFactorSignInteractor<S>>),
}

impl <S: Signable> SignInteractor<S> {
    #[allow(unused)]
    pub fn poly(interactor: Arc<dyn PolyFactorSignInteractor<S>>) -> Self {
        Self::PolyFactor(interactor)
    }

    #[allow(unused)]
    pub fn mono(interactor: Arc<dyn MonoFactorSignInteractor<S>>) -> Self {
        Self::MonoFactor(interactor)
    }
}
