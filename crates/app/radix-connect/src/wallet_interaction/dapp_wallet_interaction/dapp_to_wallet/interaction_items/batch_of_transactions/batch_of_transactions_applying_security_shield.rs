use prelude::fixture_rtm;

use crate::prelude::*;

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct BatchOfTransactionsApplyingSecurityShield {
    /// The ID of security shield being applied
    pub shield_id: SecurityStructureID,

    /// The address of the entity for which we apply the security shield.
    pub entity_address: AddressOfAccountOrPersona,

    /// This Vec will contain a single TransactionManifest if entity identified
    /// by entity_address is unsecurified, but if it is securified it will contain
    /// `RolesExercisableInTransactionManifestCombination::all().len()` many
    /// TransactionManifests
    pub transactions: Vec<UnvalidatedTransactionManifest>,
}

impl BatchOfTransactionsApplyingSecurityShield {
    pub fn new(
        shield_id: SecurityStructureID,
        entity_address: AddressOfAccountOrPersona,
        transactions: impl IntoIterator<Item = UnvalidatedTransactionManifest>,
    ) -> Self {
        Self {
            shield_id,
            entity_address,
            transactions: transactions.into_iter().collect(),
        }
    }
}
impl HasSampleValues for BatchOfTransactionsApplyingSecurityShield {
    fn sample() -> Self {
        let init_p_conf_r = TransactionManifest::new(
            fixture_rtm!("update_shield_of_persona_init_with_P_confirm_with_R"),
            NetworkID::Mainnet,
            Blobs::default(),
        )
        .unwrap();
        let init_p_conf_c = TransactionManifest::new(
            fixture_rtm!("update_shield_of_persona_init_with_P_confirm_with_C"),
            NetworkID::Mainnet,
            Blobs::default(),
        )
        .unwrap();
        let init_p_conf_t = TransactionManifest::new(
            fixture_rtm!("update_shield_of_persona_init_with_P_confirm_with_T"),
            NetworkID::Mainnet,
            Blobs::default(),
        )
        .unwrap();

        let init_r_conf_c = TransactionManifest::new(
            fixture_rtm!("update_shield_of_persona_init_with_R_confirm_with_C"),
            NetworkID::Mainnet,
            Blobs::default(),
        )
        .unwrap();
        let init_r_conf_t = TransactionManifest::new(
            fixture_rtm!("update_shield_of_persona_init_with_R_confirm_with_T"),
            NetworkID::Mainnet,
            Blobs::default(),
        )
        .unwrap();
        Self::new(
            SecurityStructureID::sample(),
            IdentityAddress::sample_mainnet().into(),
            [
                init_p_conf_r,
                init_p_conf_c,
                init_p_conf_t,
                init_r_conf_c,
                init_r_conf_t,
            ]
            .into_iter()
            .map(Into::into),
        )
    }

    fn sample_other() -> Self {
        let unsecurified = TransactionManifest::new(
            fixture_rtm!("create_access_controller_for_account"),
            NetworkID::Mainnet,
            Blobs::default(),
        )
        .unwrap();
        Self::new(SecurityStructureID::sample(), "account_rdx128dtethfy8ujrsfdztemyjk0kvhnah6dafr57frz85dcw2c8z0td87".parse().unwrap(), [
            unsecurified
        ].into_iter().map(Into::into),)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = BatchOfTransactionsApplyingSecurityShield;

    #[test]
    fn equality() {
        assert_eq!(SUT::sample(), SUT::sample());
    }

    #[test]
    fn inequality() {
        assert_ne!(SUT::sample(), SUT::sample_other());
    }
}
