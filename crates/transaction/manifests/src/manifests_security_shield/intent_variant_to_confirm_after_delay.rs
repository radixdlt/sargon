use crate::prelude::*;

#[derive(Clone, PartialEq, Eq, StdHash, Debug)]
pub struct IntentVariantToConfirmAfterDelay {
    pub role_which_initiated_recovery: RoleInitiatingRecovery,
    pub intent: TransactionIntent,
    pub entity_applying_shield: AddressOfAccountOrPersona,
    /// The time user has to wait after this intent has been broadcasted to
    /// the network before recovery can be confirmed.
    pub number_of_minutes_until_confirm_is_callable: u32,
}

impl IntentVariantToConfirmAfterDelay {
    pub fn new(
        role_which_initiated_recovery: impl Into<RoleInitiatingRecovery>,
        intent: TransactionIntent,
        entity_applying_shield: AddressOfAccountOrPersona,
        number_of_minutes_until_confirm_is_callable: u32,
    ) -> Self {
        let role_which_initiated_recovery =
            role_which_initiated_recovery.into();
        Self {
            role_which_initiated_recovery,
            intent,
            entity_applying_shield,
            number_of_minutes_until_confirm_is_callable,
        }
    }
}
