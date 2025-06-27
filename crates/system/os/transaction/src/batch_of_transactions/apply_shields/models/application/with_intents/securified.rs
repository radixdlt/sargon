use crate::prelude::*;

pub type AbstractSecurityShieldApplicationForSecurifiedEntityWithIntent<
    Entity,
> = AbstractSecurityShieldApplicationForSecurifiedEntityWithPayload<
    Entity,
    TransactionIntent,
>;
