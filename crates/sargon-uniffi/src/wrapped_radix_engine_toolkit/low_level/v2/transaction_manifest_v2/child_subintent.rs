use crate::prelude::*;
use sargon::ChildSubintent as InternalChildSubintent;

#[derive(Clone, PartialEq, Eq, InternalConversion, uniffi::Record)]
pub struct ChildSubintent {
    pub hash: SubintentHash,
}