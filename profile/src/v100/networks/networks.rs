use crate::prelude::*;

/// An ordered mapping of NetworkID -> `Profile.Network`, containing
/// all the users Accounts, Personas and AuthorizedDapps the user
/// has created and interacted with on this network.
pub type Networks = IdentifiedVecVia<Network>;

// Constructors
impl Networks {
    /// Instantiates a new collection of networks from
    /// and iterator.
    pub fn with_networks<I>(networks: I) -> Self
    where
        I: IntoIterator<Item = Network>,
    {
        Self::from_iter(networks)
    }

    /// Instantiates a new network collection with the provided
    /// `network`.
    pub fn with_network(network: Network) -> Self {
        Self::with_networks([network].into_iter())
    }
}

impl Networks {
    pub fn get_account(&self, address: &AccountAddress) -> Option<Account> {
        self.get(&address.network_id)
            .and_then(|n| n.accounts.get_account_by_address(address))
            .cloned()
    }

    /// Returns a clone of the updated account if found, else None.
    pub fn update_account<F>(&mut self, address: &AccountAddress, mut mutate: F) -> Option<Account>
    where
        F: FnMut(&mut Account) -> (),
    {
        self.update_with(&address.network_id, |n| {
            _ = n.update_account(address, |a| mutate(a))
        });
        self.get_account(address)
    }
}

impl Networks {
    pub fn content_hint(&self) -> ContentHint {
        let number_of_accounts = self.iter().fold(0, |acc, x| acc + x.accounts.len());
        ContentHint::with_counters(number_of_accounts, 0, self.len())
    }
}

// Trait: Default
impl Default for Networks {
    /// Instantiates a new empty networks collection.
    fn default() -> Self {
        Self::new()
    }
}

impl HasPlaceholder for Networks {
    /// A placeholder used to facilitate unit tests.
    fn placeholder() -> Self {
        Self::with_networks(
            [
                Network::placeholder_mainnet(),
                Network::placeholder_stokenet(),
            ]
            .into_iter(),
        )
    }

    /// A placeholder used to facilitate unit tests.
    fn placeholder_other() -> Self {
        Self::with_network(Network::placeholder_other())
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::*;

    #[test]
    fn default_is_empty() {
        assert_eq!(Networks::default().len(), 0)
    }

    #[test]
    fn inequality() {
        assert_ne!(Networks::placeholder(), Networks::placeholder_other());
    }

    #[test]
    fn equality() {
        assert_eq!(Networks::placeholder(), Networks::placeholder());
        assert_eq!(Networks::placeholder_other(), Networks::placeholder_other());
    }

    #[test]
    fn duplicates_are_prevented() {
        assert_eq!(
            Networks::from_iter([Network::placeholder(), Network::placeholder()].into_iter()).len(),
            1
        )
    }

    #[test]
    fn duplicates_are_prevented_and_first_added_is_retained() {
        let mut sut =
            Networks::from_iter([Network::new(
                NetworkID::Mainnet,
                Accounts::from_iter([
                    Account::placeholder_mainnet_alice(),
                    Account::placeholder_mainnet_bob(),
                ]),
            )]);
        assert_eq!(
            sut.append(Network::new(
                NetworkID::Mainnet,
                Accounts::from_iter([Account::placeholder_mainnet_carol()]),
            ))
            .0,
            false
        );

        assert_eq!(
            sut.get(&NetworkID::Mainnet).unwrap().accounts.items(),
            [
                Account::placeholder_mainnet_alice(),
                Account::placeholder_mainnet_bob()
            ]
        );
    }

    #[test]
    fn update_account() {
        let mut sut = Networks::placeholder();
        let id = &NetworkID::Mainnet;
        let account_address = Account::placeholder().address;
        assert_eq!(
            sut.get(id)
                .unwrap()
                .accounts
                .get(&account_address)
                .unwrap()
                .display_name
                .value,
            "Alice"
        );

        sut.update_account(&account_address, |a| {
            a.display_name = DisplayName::new("Stella").unwrap()
        });

        assert_eq!(
            sut.get(id)
                .unwrap()
                .accounts
                .get(&account_address)
                .unwrap()
                .display_name
                .value,
            "Stella"
        );
    }

    #[test]
    fn update_account_unknown_network() {
        let mut sut = Networks::placeholder();
        let id = &NetworkID::Mainnet;
        let account_address = Account::placeholder_nebunet().address;
        assert_eq!(sut.get(id).unwrap().accounts.get(&account_address), None);

        assert!(sut
            .update_account(&account_address, |a| {
                a.display_name = DisplayName::new("will fail").unwrap()
            })
            .is_none());

        // Assert unchanged
        assert_eq!(sut, Networks::placeholder());
    }

    #[test]
    fn update_account_unknown_account() {
        let mut sut = Networks::placeholder();
        let id = &NetworkID::Mainnet;
        let account_address = Account::placeholder_mainnet_carol().address;
        assert_eq!(sut.get(id).unwrap().accounts.get(&account_address), None);

        assert!(sut
            .update_account(&account_address, |a| {
                a.display_name = DisplayName::new("will fail").unwrap()
            })
            .is_none());

        // Assert unchanged
        assert_eq!(sut, Networks::placeholder());
    }

    #[test]
    fn with_network() {
        let network = Network::new(
            NetworkID::Mainnet,
            Accounts::with_account(Account::placeholder_mainnet()),
        );
        assert_eq!(Networks::with_network(network).len(), 1);
    }

    #[test]
    fn content_hint() {
        assert_eq!(
            Networks::placeholder().content_hint(),
            ContentHint::with_counters(4, 0, 2)
        );
    }

    #[test]
    fn json_roundtrip() {
        let sut = Networks::placeholder();
        assert_eq_after_json_roundtrip(
            &sut,
            r#"
            [
                {
                    "networkID": 1,
                    "accounts": [
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
                            "flags": [],
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
                            "flags": [],
                            "address": "account_rdx129a9wuey40lducsf6yu232zmzk5kscpvnl6fv472r0ja39f3hced69"
                        }
                    ]
                },
                {
                    "networkID": 2,
                    "accounts": [
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
                            "displayName": "Carol",
                            "onLedgerSettings": {
                                "thirdPartyDeposits": {
                                    "depositRule": "acceptAll",
                                    "assetsExceptionList": [],
                                    "depositorsAllowList": []
                                }
                            },
                            "flags": [],
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
                            "flags": [],
                            "displayName": "Diana",
                            "onLedgerSettings": {
                                "thirdPartyDeposits": {
                                    "depositRule": "acceptAll",
                                    "assetsExceptionList": [],
                                    "depositorsAllowList": []
                                }
                            },
                            "flags": [],
                            "address": "account_tdx_2_129663ef7fj8azge3y6sl73lf9vyqt53ewzlf7ul2l76mg5wyqlqlpr"
                        }
                    ]
                }
            ]
            "#,
        );
    }
}
