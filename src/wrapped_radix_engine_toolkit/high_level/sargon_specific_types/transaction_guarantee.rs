use crate::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq, Hash, uniffi::Record)]
pub struct TransactionGuarantee {
    pub amount: Decimal192,
    pub instruction_index: u64,
    pub resource_address: ResourceAddress,
    pub resource_divisibility: Option<i32>,
}

impl TransactionGuarantee {
    pub fn new(
        amount: impl Into<Decimal192>,
        instruction_index: u64,
        resource_address: ResourceAddress,
        resource_divisibility: impl Into<Option<i32>>,
    ) -> Self {
        Self {
            amount: amount.into(),
            instruction_index,
            resource_address,
            resource_divisibility: resource_divisibility.into(),
        }
    }
}
