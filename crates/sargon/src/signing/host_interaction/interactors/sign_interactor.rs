use crate::prelude::*;

/// An interactor which can sign transactions - either in poly or mono.
pub enum SignInteractor<SP: SignablePayload> {
    PolyFactor(Arc<dyn PolyFactorSignInteractor<SP>>),
    MonoFactor(Arc<dyn MonoFactorSignInteractor<SP>>),
}

impl <SP: SignablePayload> SignInteractor<SP> {
    #[allow(unused)]
    pub fn poly(interactor: Arc<dyn PolyFactorSignInteractor<SP>>) -> Self {
        Self::PolyFactor(interactor)
    }

    #[allow(unused)]
    pub fn mono(interactor: Arc<dyn MonoFactorSignInteractor<SP>>) -> Self {
        Self::MonoFactor(interactor)
    }
}
