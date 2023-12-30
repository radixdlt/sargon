use serde::{Deserialize, Serialize};

use crate::v100::{Account, AccountAddress, ContentHint};

#[cfg(any(test, feature = "placeholder"))]
use crate::HasPlaceholder;

use super::Network;
use identified_vec::Identifiable;

/// An ordered mapping of NetworkID -> `Profile.Network`, containing
/// all the users Accounts, Personas and AuthorizedDapps the user
/// has created and interacted with on this network.
#[derive(Serialize, Deserialize, Clone, Hash, Debug, PartialEq, Eq, uniffi::Record)]
#[serde(transparent)]
pub struct Networks {
    // FIXME: Now
    pub vec: Vec<Network>,
}
impl Networks {
    pub fn new() -> Self {
        Self { vec: Vec::new() }
    }
    pub fn from_iter<I>(iter: I) -> Self
    where
        I: IntoIterator<Item = Network>,
    {
        Self {
            vec: Vec::from_iter(iter),
        }
    }
    pub fn append(&mut self, network: Network) {
        if self.vec.iter().any(|x| x.id() == network.id()) {
            return;
        }
        self.vec.push(network);
    }
    pub fn len(&self) -> usize {
        self.vec.len()
    }
}

// pub type Networks = IdentifiedVecVia<Network>;

// Constructors
impl Networks {
    /// Instantiates a new collection of networks from
    /// and iterator.
    pub fn with_networks<I>(networks: I) -> Self
    where
        I: Iterator<Item = Network>,
    {
        // Self::from_identified_vec_of(IdentifiedVecOf::from_iter(networks))
        Self::from_iter(networks)
    }

    /// Instantiates a new network collection with the provided
    /// `network`.
    pub fn with_network(network: Network) -> Self {
        Self::with_networks([network].into_iter())
    }
}

impl Networks {
    // pub fn update_mut<F, R>(&mut self, mutate: F) -> R
    // where
    //     F: Fn(&mut Self) -> R,
    // {
    //     mutate(self)
    // }

    // pub fn update<F, R>(&self, mutate: F) -> R
    // where
    //     F: Fn(&Self) -> R,
    // {
    //     mutate(self)
    // }

    /// Returns `false` if no account with `address` was found, otherwise if found,
    /// the account gets updated by `mutate` closure and this function returns
    /// `true`.
    pub fn update_account<F>(&mut self, _address: &AccountAddress, mut _mutate: F) -> bool
    where
        F: FnMut(&Account) -> (),
    {
        // let mut updated_account = false;
        // let updated_network = self.update_with(&address.network_id(), |n| {
        //     updated_account = n.update_account(address, |a| mutate(a))
        // });
        // if !updated_account {
        //     return false;
        // }

        // assert!(
        //     updated_network,
        //     "Strange! Update an account, which was not on this network?"
        // );

        // return updated_account && updated_network;
        todo!()
    }
}

impl Networks {
    pub fn content_hint(&self) -> ContentHint {
        // let number_of_accounts = self
        //     .vec
        //     .iter()
        //     .fold(0, |acc, x| acc + x.accounts().vec.len());
        // ContentHint::with_counters(number_of_accounts, 0, self.vec.len())
        todo!()
    }
}

// Trait: Default
impl Default for Networks {
    /// Instantiates a new empty networks collection.
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(any(test, feature = "placeholder"))]
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
    use crate::{assert_eq_after_json_roundtrip, HasPlaceholder, Networks};

    /*

        #[test]
        fn default_is_empty() {
            assert_eq!(Networks::default().len(), 0)
        }

        #[test]
        fn inequality() {
            assert_ne!(Networks::placeholder(), Networks::placeholder_other());
        }

        #[test]
        fn update_account() {
            let mut sut = Networks::placeholder();
            let id = &NetworkID::Mainnet;
            let account_address = Account::placeholder().address();
            assert_eq!(
                sut.get(id)
                    .unwrap()
                    .accounts()
                    .get(&account_address)
                    .unwrap()
                    .display_name(),
                "Alice"
            );
            assert!(sut.update_account(&account_address, |a| _ =
                a.set_display_name(DisplayName::new("Stella").unwrap())));
            assert_eq!(
                sut.get(id)
                    .unwrap()
                    .accounts()
                    .get(&account_address)
                    .unwrap()
                    .display_name(),
                "Stella"
            );
        }

        #[test]
        fn update_account_unknown_network() {
            let mut sut = Networks::placeholder();
            let id = &NetworkID::Mainnet;
            let account_address = Account::placeholder_nebunet().address();
            assert_eq!(sut.get(id).unwrap().accounts().get(&account_address), None);
            assert_eq!(
                sut.update_account(&account_address, |a| _ =
                    a.set_display_name(DisplayName::new("Will fail").unwrap())),
                false
            );
            // Assert unchanged
            assert_eq!(sut, Networks::placeholder());
        }

        #[test]
        fn update_account_unknown_account() {
            let mut sut = Networks::placeholder();
            let id = &NetworkID::Mainnet;
            let account_address = Account::placeholder_mainnet_carol().address();
            assert_eq!(sut.get(id).unwrap().accounts().get(&account_address), None);
            assert_eq!(
                sut.update_account(&account_address, |a| _ =
                    a.set_display_name(DisplayName::new("Will fail").unwrap())),
                false
            );
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
    */
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
