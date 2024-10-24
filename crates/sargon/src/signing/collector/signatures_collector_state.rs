use crate::prelude::*;

#[derive(derive_more::Debug)]
#[debug("{:#?}", petitions.borrow())]
pub(super) struct SignaturesCollectorState<S: Signable> {
    pub(super) petitions: RefCell<Petitions<S>>,
}
impl <S: Signable> SignaturesCollectorState<S> {
    pub(crate) fn new(petitions: Petitions<S>) -> Self {
        Self {
            petitions: RefCell::new(petitions),
        }
    }
}
