use crate::prelude::*;

/// An ordered set of [`Account`]s on a specific network, most commonly
/// the set is non-empty, since wallets guide user to create a first
/// Account.
pub type Accounts = OrderedMap<Account>;

impl HasSampleValues for Accounts {
    /// A sample used to facilitate unit tests.
    fn sample() -> Self {
        Self::sample_mainnet()
    }

    /// A sample used to facilitate unit tests.
    fn sample_other() -> Self {
        Self::sample_stokenet()
    }
}

impl Accounts {
    /// A sample used to facilitate unit tests.
    pub fn sample_mainnet() -> Self {
        Self::from_iter([
            Account::sample_mainnet(),
            Account::sample_mainnet_other(),
        ])
    }

    /// A sample used to facilitate unit tests.
    pub fn sample_stokenet() -> Self {
        Self::from_iter([
            Account::sample_stokenet_nadia(),
            Account::sample_stokenet_olivia(),
        ])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = Accounts;

    #[test]
    fn default_is_empty() {
        assert_eq!(SUT::default().len(), 0);
    }

    #[test]
    fn inequality() {
        assert_ne!(SUT::sample(), SUT::sample_other());
    }

    #[test]
    fn equality() {
        assert_eq!(SUT::sample(), SUT::sample());
        assert_eq!(SUT::sample_other(), SUT::sample_other());
    }

    #[test]
    fn duplicates_are_prevented() {
        assert_eq!(
            SUT::from_iter([Account::sample(), Account::sample()].into_iter())
                .len(),
            1
        )
    }

    #[test]
    fn with_one() {
        assert_eq!(SUT::just(Account::sample()).len(), 1)
    }

    #[test]
    fn get_all() {
        assert_eq!(SUT::sample().get_all().len(), 2);
    }

    #[test]
    fn get_by_address() {
        let address = AccountAddress::sample();
        let account = Account::sample_with_values(
            address,
            DisplayName::default(),
            AppearanceID::default(),
        );
        let accounts = SUT::just(account.clone());
        assert_eq!(accounts.get_id(&address), Some(&account));
    }

    #[test]
    fn json_roundtrip_stokenet() {
        let sut = SUT::sample_stokenet();
        assert_eq_after_json_roundtrip(
            &sut,
            r#"
            [
                {
                    "securityState": {
                        "unsecuredEntityControl": {
                            "transactionSigning": {
                                "badge": {
                                    "virtualSource": {
                                        "hierarchicalDeterministicPublicKey": {
                                            "publicKey": {
                                                "curve": "curve25519",
                                                "compressedData": "18c7409458a82281711b668f833b0485e8fb58a3ceb8a728882bf6b83d3f06a9"
                                            },
                                            "derivationPath": {
                                                "scheme": "cap26",
                                                "path": "m/44H/1022H/2H/525H/1460H/0H"
                                            }
                                        },
                                        "discriminator": "hierarchicalDeterministicPublicKey"
                                    },
                                    "discriminator": "virtualSource"
                                },
                                "factorSourceID": {
                                    "fromHash": {
                                        "kind": "device",
                                        "body": "3c986ebf9dcd9167a97036d3b2c997433e85e6cc4e4422ad89269dac7bfea240"
                                    },
                                    "discriminator": "fromHash"
                                }
                            }
                        },
                        "discriminator": "unsecured"
                    },
                    "networkID": 2,
                    "appearanceID": 0,
                    "flags": [],
                    "displayName": "Nadia",
                    "onLedgerSettings": {
                        "thirdPartyDeposits": {
                            "depositRule": "acceptAll",
                            "assetsExceptionList": [],
                            "depositorsAllowList": []
                        }
                    },
                    "address": "account_tdx_2_1289zm062j788dwrjefqkfgfeea5tkkdnh8htqhdrzdvjkql4kxceql"
                },
                {
                    "securityState": {
                        "unsecuredEntityControl": {
                            "transactionSigning": {
                                "badge": {
                                    "virtualSource": {
                                        "hierarchicalDeterministicPublicKey": {
                                            "publicKey": {
                                                "curve": "curve25519",
                                                "compressedData": "26b3fd7f65f01ff8e418a56722fde9cc6fc18dc983e0474e6eb6c1cf3bd44f23"
                                            },
                                            "derivationPath": {
                                                "scheme": "cap26",
                                                "path": "m/44H/1022H/2H/525H/1460H/1H"
                                            }
                                        },
                                        "discriminator": "hierarchicalDeterministicPublicKey"
                                    },
                                    "discriminator": "virtualSource"
                                },
                                "factorSourceID": {
                                    "fromHash": {
                                        "kind": "device",
                                        "body": "3c986ebf9dcd9167a97036d3b2c997433e85e6cc4e4422ad89269dac7bfea240"
                                    },
                                    "discriminator": "fromHash"
                                }
                            }
                        },
                        "discriminator": "unsecured"
                    },
                    "networkID": 2,
                    "appearanceID": 1,
                    "flags": ["deletedByUser"],
                    "displayName": "Olivia",
                    "onLedgerSettings": {
                        "thirdPartyDeposits": {
                            "depositRule": "acceptAll",
                            "assetsExceptionList": [],
                            "depositorsAllowList": []
                        }
                    },
                    "address": "account_tdx_2_129663ef7fj8azge3y6sl73lf9vyqt53ewzlf7ul2l76mg5wyqlqlpr"
                }
            ]
            "#,
        );
    }

    #[test]
    fn json_roundtrip_mainnet() {
        let sut = Accounts::sample_mainnet();
        assert_eq_after_json_roundtrip(
            &sut,
            r#"
            [
                {
                    "securityState": {
                        "unsecuredEntityControl": {
                            "transactionSigning": {
                                "badge": {
                                    "virtualSource": {
                                        "hierarchicalDeterministicPublicKey": {
                                            "publicKey": {
                                                "curve": "curve25519",
                                                "compressedData": "d24cc6af91c3f103d7f46e5691ce2af9fea7d90cfb89a89d5bba4b513b34be3b"
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
                                        "body": "3c986ebf9dcd9167a97036d3b2c997433e85e6cc4e4422ad89269dac7bfea240"
                                    },
                                    "discriminator": "fromHash"
                                }
                            }
                        },
                        "discriminator": "unsecured"
                    },
                    "networkID": 1,
                    "appearanceID": 0,
                    "flags": [],
                    "displayName": "Alice",
                    "onLedgerSettings": {
                        "thirdPartyDeposits": {
                            "depositRule": "acceptAll",
                            "assetsExceptionList": [],
                            "depositorsAllowList": []
                        }
                    },
                    "address": "account_rdx12yy8n09a0w907vrjyj4hws2yptrm3rdjv84l9sr24e3w7pk7nuxst8"
                },
                {
                    "securityState": {
                        "unsecuredEntityControl": {
                            "transactionSigning": {
                                "badge": {
                                    "virtualSource": {
                                        "hierarchicalDeterministicPublicKey": {
                                            "publicKey": {
                                                "curve": "curve25519",
                                                "compressedData": "08740a2fd178c40ce71966a6537f780978f7f00548cfb59196344b5d7d67e9cf"
                                            },
                                            "derivationPath": {
                                                "scheme": "cap26",
                                                "path": "m/44H/1022H/1H/525H/1460H/1H"
                                            }
                                        },
                                        "discriminator": "hierarchicalDeterministicPublicKey"
                                    },
                                    "discriminator": "virtualSource"
                                },
                                "factorSourceID": {
                                    "fromHash": {
                                        "kind": "device",
                                        "body": "3c986ebf9dcd9167a97036d3b2c997433e85e6cc4e4422ad89269dac7bfea240"
                                    },
                                    "discriminator": "fromHash"
                                }
                            }
                        },
                        "discriminator": "unsecured"
                    },
                    "networkID": 1,
                    "appearanceID": 1,
                    "flags": [],
                    "displayName": "Bob",
                    "onLedgerSettings": {
                        "thirdPartyDeposits": {
                            "depositRule": "acceptAll",
                            "assetsExceptionList": [],
                            "depositorsAllowList": []
                        }
                    },
                    "address": "account_rdx129a9wuey40lducsf6yu232zmzk5kscpvnl6fv472r0ja39f3hced69"
                }
            ]
            "#,
        );
    }
}
