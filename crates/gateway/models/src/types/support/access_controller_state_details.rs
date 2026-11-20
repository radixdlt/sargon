use crate::prelude::*;

#[derive(Serialize, Deserialize, Clone, PartialEq, Eq, Debug)]
pub struct AccessControllerStateDetails {
    pub address: AccessControllerAddress,
    pub state: AccessControllerFieldStateValue,
    pub xrd_balance: Decimal192,

    /// The access controller's role assignments, which hold its current
    /// rules, see [`Self::rule_set`]. `None` in details cached before they
    /// were read.
    #[serde(default)]
    pub role_assignments: Option<ComponentEntityRoleAssignments>,

    /// The account or persona whose owner badge is in the access
    /// controller's vault, i.e. the one it controls. `None` in details cached
    /// before it was read.
    #[serde(default)]
    pub controlled_entity: Option<AddressOfAccountOrPersona>,
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
            role_assignments: None,
            controlled_entity: None,
        }
    }

    /// The access controller's current rules.
    pub fn rule_set(&self) -> Result<ScryptoRuleSet> {
        self.role_assignments
            .clone()
            .ok_or_else(|| CommonError::Unknown {
                error_message: format!(
                    "No role assignments for {}",
                    self.address
                ),
            })?
            .try_into()
    }
}

impl HasSampleValues for AccessControllerStateDetails {
    fn sample() -> Self {
        AccessControllerStateDetails::new(
            AccessControllerAddress::sample(),
            AccessControllerFieldStateValue::sample(),
            Decimal192::zero(),
        )
    }

    fn sample_other() -> Self {
        AccessControllerStateDetails::new(
            AccessControllerAddress::sample_other(),
            AccessControllerFieldStateValue::sample_other(),
            Decimal192::zero(),
        )
    }
}
