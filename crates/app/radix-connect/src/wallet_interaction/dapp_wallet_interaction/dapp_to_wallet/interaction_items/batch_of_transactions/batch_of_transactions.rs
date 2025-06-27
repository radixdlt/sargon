use prelude::fixture_rtm;

use crate::prelude::*;

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct DappToWalletInteractionBatchOfTransactions {
    pub transactions: Vec<UnvalidatedTransactionManifest>,
}

impl DappToWalletInteractionBatchOfTransactions {
    pub fn new(
        transactions: impl IntoIterator<Item = UnvalidatedTransactionManifest>,
    ) -> Self {
        Self {
            transactions: transactions.into_iter().collect(),
        }
    }
}

impl HasSampleValues for DappToWalletInteractionBatchOfTransactions {
    fn sample() -> Self {
        let init_p_conf_r = TransactionManifest::new(
            fixture_rtm!("update_shield_of_persona_init_with_R_confirm_with_P"),
            NetworkID::Mainnet,
            Blobs::default(),
        )
        .unwrap();
        let unsecurified = TransactionManifest::new(
            fixture_rtm!("create_access_controller_for_account"),
            NetworkID::Mainnet,
            Blobs::default(),
        )
        .unwrap();
        Self::new(
            [init_p_conf_r, unsecurified]
                .map(UnvalidatedTransactionManifest::from),
        )
    }

    fn sample_other() -> Self {
        let init_p_conf_c = TransactionManifest::new(
            fixture_rtm!("update_shield_of_persona_init_with_R_confirm_with_P"),
            NetworkID::Mainnet,
            Blobs::default(),
        )
        .unwrap();

        Self::new([init_p_conf_c].map(UnvalidatedTransactionManifest::from))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = DappToWalletInteractionBatchOfTransactions;

    #[test]
    fn equality() {
        assert_eq!(SUT::sample(), SUT::sample());
    }

    #[test]
    fn inequality() {
        assert_ne!(SUT::sample(), SUT::sample_other());
    }
}
