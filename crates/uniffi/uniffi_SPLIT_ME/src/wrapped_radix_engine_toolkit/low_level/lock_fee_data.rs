use crate::prelude::*;
use sargon::LockFeeData as InternalLockFeeData;

#[derive(Clone, PartialEq, Eq, uniffi::Record)]
pub struct LockFeeData {
    pub fee_payer_address: AccountAddress,
    pub access_controller_address: Option<AccessControllerAddress>,
    pub fee: Option<Decimal192>,
}

impl From<InternalLockFeeData> for LockFeeData {
    fn from(value: InternalLockFeeData) -> Self {
        Self {
            fee_payer_address: value.fee_payer_address.into(),
            access_controller_address: value
                .access_controller_address
                .map(Into::into),
            fee: Some(value.fee().into()),
        }
    }
}

impl IntoInternal<LockFeeData, InternalLockFeeData> for LockFeeData {
    fn into_internal(self) -> InternalLockFeeData {
        match (self.access_controller_address, self.fee) {
            (Some(access_controller_address), maybe_fee) => {
                let fee = maybe_fee.unwrap_or_else(|| new_decimal_from_u32(25));
                InternalLockFeeData::new_with_securified_fee_payer(
                    self.fee_payer_address.into(),
                    access_controller_address.into(),
                    fee.into(),
                )
            }
            (None, maybe_fee) => {
                let fee = maybe_fee.unwrap_or_else(|| new_decimal_from_u32(25));
                InternalLockFeeData::new_with_unsecurified_fee_payer(
                    self.fee_payer_address.into(),
                    fee.into(),
                )
            }
        }
    }
}

impl From<LockFeeData> for InternalLockFeeData {
    fn from(value: LockFeeData) -> Self {
        value.into_internal()
    }
}
