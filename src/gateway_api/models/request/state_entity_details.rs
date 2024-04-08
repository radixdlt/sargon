use crate::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct StateEntityDetailsRequest {
    pub(crate) addresses: Vec<Address>,
}
