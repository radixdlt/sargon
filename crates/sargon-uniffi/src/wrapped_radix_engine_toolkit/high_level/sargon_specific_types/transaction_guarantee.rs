use crate::prelude::*;
use sargon::TransactionGuarantee as InternalTransactionGuarantee;

#[derive(Clone, Debug, PartialEq, Eq, Hash, uniffi::Record)]
pub struct TransactionGuarantee {
    /// The guaranteed amount to be obtained on this transaction. For manifest & display purposes.
    pub amount: Decimal192,

    /// The percentage the user has selected, which generated the `amount`. For display purposes only.
    pub percentage: Decimal192,
    pub instruction_index: u64,
    pub resource_address: ResourceAddress,
    pub resource_divisibility: Option<u8>,
}

impl From<InternalTransactionGuarantee> for TransactionGuarantee {
    fn from(value: InternalTransactionGuarantee) -> Self {
        Self {
            amount: value.amount.into(),
            percentage: value.percentage.into(),
            instruction_index: value.instruction_index,
            resource_address: value.resource_address.into(),
            resource_divisibility: value.resource_divisibility,
        }
    }
}

impl Into<InternalTransactionGuarantee> for TransactionGuarantee {
    fn into(self) -> InternalTransactionGuarantee {
        InternalTransactionGuarantee {
            amount: self.amount.into(),
            percentage: self.percentage.into(),
            instruction_index: self.instruction_index,
            resource_address: self.resource_address.into(),
            resource_divisibility: self.resource_divisibility,
        }
    }
}