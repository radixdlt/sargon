use crate::prelude::*;

#[derive(derive_more::Debug)]
#[debug("{:#?}", petitions.borrow())]
pub(super) struct SignaturesCollectorState {
    pub(super) petitions: RefCell<Petitions>,
}
impl SignaturesCollectorState {
    pub(crate) fn new(petitions: Petitions) -> Self {
        Self {
            petitions: RefCell::new(petitions),
        }
    }
}
