use crate::prelude::*;
use sargon::AccessControllerRuleSetForDisplay as InternalAccessControllerRuleSetForDisplay;

/// What each role of an access controller requires, as shown to the user.
#[derive(Clone, PartialEq, Eq, Hash, InternalConversion, uniffi::Record)]
pub struct AccessControllerRuleSetForDisplay {
    pub primary_role: AccessRequirementForDisplay,
    pub recovery_role: AccessRequirementForDisplay,
    pub confirmation_role: AccessRequirementForDisplay,
}

#[uniffi::export]
pub fn new_access_controller_rule_set_for_display_sample(
) -> AccessControllerRuleSetForDisplay {
    InternalAccessControllerRuleSetForDisplay::sample().into()
}

#[uniffi::export]
pub fn new_access_controller_rule_set_for_display_sample_other(
) -> AccessControllerRuleSetForDisplay {
    InternalAccessControllerRuleSetForDisplay::sample_other().into()
}

decl_conversion_tests_for!(AccessControllerRuleSetForDisplay);
