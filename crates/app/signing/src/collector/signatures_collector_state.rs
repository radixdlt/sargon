use crate::prelude::*;

#[derive(derive_more::Debug)]
#[debug("{:#?}", petitions.read().expect("SignaturesCollectorState lock should not have been poisoned."))]
pub(super) struct SignaturesCollectorState<S: Signable> {
    pub(super) petitions: RwLock<Petitions<S>>,
}
impl<S: Signable> SignaturesCollectorState<S> {
    pub(crate) fn new(petitions: Petitions<S>) -> Self {
        Self {
            petitions: RwLock::new(petitions),
        }
    }
}
