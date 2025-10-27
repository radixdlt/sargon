use crate::prelude::*;

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct AccessControllerStateDetails {
    pub address: AccessControllerAddress,
    pub state: AccessControllerFieldStateValue,
    pub xrd_balance: Decimal192,
}

impl AccessControllerStateDetails {
    pub fn new(
        address: AccessControllerAddress,
        state: AccessControllerFieldStateValue,
        xrd_balance: impl Into<Decimal192>,
    ) -> Self {
        Self {
            address,
            state,
            xrd_balance: xrd_balance.into(),
        }
    }
}
