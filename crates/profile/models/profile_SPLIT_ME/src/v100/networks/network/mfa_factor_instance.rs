use crate::prelude::*;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[serde(rename_all = "camelCase")]
pub struct MFAFactorInstance {
    /// The `FactorInstance` used for MFA
    #[serde(rename = "factorInstance")]
    pub factor_instance: FactorInstance,
}

impl MFAFactorInstance {
    pub fn new(factor_instance: FactorInstance) -> Self {
        Self { factor_instance }
    }
}

impl Identifiable for MFAFactorInstance {
    type ID = PublicKey;
    fn id(&self) -> Self::ID {
        self.factor_instance.id()
    }
}

impl From<FactorInstance> for MFAFactorInstance {
    fn from(factor_instance: FactorInstance) -> Self {
        Self::new(factor_instance)
    }
}

impl IsNetworkAware for MFAFactorInstance {
    fn network_id(&self) -> NetworkID {
        self.factor_instance.badge.network_id()
    }
}

impl MFAFactorInstance {
    pub fn sample_mainnet_account_securified_idx_0() -> Self {
        FactorInstance::from(HierarchicalDeterministicFactorInstance::sample_with_key_kind_entity_kind_on_network_and_hardened_index(
            NetworkID::Mainnet,
            CAP26KeyKind::TransactionSigning,
            CAP26EntityKind::Account,
            Hardened::Securified(SecurifiedU30::ZERO),
        )).into()
    }
    pub fn sample_mainnet_account_securified_idx_1() -> Self {
        FactorInstance::from(HierarchicalDeterministicFactorInstance::sample_with_key_kind_entity_kind_on_network_and_hardened_index(
            NetworkID::Mainnet,
            CAP26KeyKind::TransactionSigning,
            CAP26EntityKind::Account,
            Hardened::Securified(SecurifiedU30::ONE),
        )).into()
    }

    pub fn sample_stokenet_account_securified_idx_0() -> Self {
        FactorInstance::from(HierarchicalDeterministicFactorInstance::sample_with_key_kind_entity_kind_on_network_and_hardened_index(
            NetworkID::Stokenet,
            CAP26KeyKind::TransactionSigning,
            CAP26EntityKind::Account,
            Hardened::Securified(SecurifiedU30::ZERO),
        )).into()
    }
    pub fn sample_stokenet_account_securified_idx_1() -> Self {
        FactorInstance::from(HierarchicalDeterministicFactorInstance::sample_with_key_kind_entity_kind_on_network_and_hardened_index(
            NetworkID::Stokenet,
            CAP26KeyKind::TransactionSigning,
            CAP26EntityKind::Account,
            Hardened::Securified(SecurifiedU30::ONE),
        )).into()
    }
}

impl HasSampleValues for MFAFactorInstance {
    /// A sample used to facilitate unit tests.
    fn sample() -> Self {
        Self::new(FactorInstance::sample())
    }

    /// A sample used to facilitate unit tests.
    fn sample_other() -> Self {
        Self::new(FactorInstance::sample_other())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = MFAFactorInstance;

    #[test]
    fn equality() {
        assert_eq!(SUT::sample(), SUT::sample());
        assert_eq!(SUT::sample_other(), SUT::sample_other());
    }

    #[test]
    fn inequality() {
        assert_ne!(SUT::sample(), SUT::sample_other());
    }

    #[test]
    fn json_roundtrip() {
        let model = SUT::sample();
        assert_eq_after_json_roundtrip(
            &model,
            r#"
            {
                "factorInstance": {
                    "badge": {
                        "virtualSource": {
                            "hierarchicalDeterministicPublicKey": {
                                "publicKey": {
                                    "curve": "curve25519",
                                    "compressedData": "c05f9fa53f203a01cbe43e89086cae29f6c7cdd5a435daa9e52b69e656739b36"
                                },
                                "derivationPath": {
                                    "scheme": "cap26",
                                    "path": "m/44H/1022H/1H/525H/1460H/0H"
                                }
                            },
                            "discriminator": "hierarchicalDeterministicPublicKey"
                        },
                        "discriminator": "virtualSource"
                    },
                    "factorSourceID": {
                        "fromHash": {
                            "kind": "device",
                            "body": "f1a93d324dd0f2bff89963ab81ed6e0c2ee7e18c0827dc1d3576b2d9f26bbd0a"
                        },
                        "discriminator": "fromHash"
                    }
                }
			}
            "#,
        );
    }
}
