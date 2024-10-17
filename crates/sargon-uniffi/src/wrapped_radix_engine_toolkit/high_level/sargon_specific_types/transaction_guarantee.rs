use crate::prelude::*;
use sargon::TransactionGuarantee as InternalTransactionGuarantee;

#[derive(Clone, PartialEq, Eq, Hash, InternalConversion, uniffi::Record)]
pub struct TransactionGuarantee {
    /// The guaranteed amount to be obtained on this transaction. For manifest & display purposes.
    pub amount: Decimal192,

    /// The percentage the user has selected, which generated the `amount`. For display purposes only.
    pub percentage: Decimal192,
    pub instruction_index: u64,
    pub resource_address: ResourceAddress,
    pub resource_divisibility: Option<u8>,
}