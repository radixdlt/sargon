use crate::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq, Hash, uniffi::Record)]
pub struct TransactionGuarantee {
    pub amount: Decimal192,
    pub instruction_index: u64,
    pub resource_address: ResourceAddress,
    pub resource_divisibility: Option<i64>,
}
