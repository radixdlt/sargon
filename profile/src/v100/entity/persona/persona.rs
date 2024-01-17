use std::{cmp::Ordering, fmt::Display};

use identified_vec::Identifiable;
use serde::{Deserialize, Serialize};

use crate::prelude::*;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Hash, Eq, uniffi::Record)]
#[serde(rename_all = "camelCase")]
pub struct Persona {
    /// The ID of the network this account can be used with.
    #[serde(rename = "networkID")]
    pub network_id: NetworkID,

    /// The address of an identity, used by Personas, a bech32 encoding of a public key hash
    /// that starts with the prefix `"identity_"`, dependent on NetworkID, meaning the same
    /// public key used for two IdentityAddresses on two different networks will not have
    /// the same address.
    pub address: IdentityAddress,

    /// An off-ledger display name or description chosen by the user when they
    /// created this persona.
    pub display_name: DisplayName,

    /// Describes the state this Persona is in, in regards to how
    /// the user controls it, i.e. if it is controlled by a single factor (private key)
    ///  or an `AccessController` with a potential Multi-Factor setup.
    pub security_state: EntitySecurityState,

    /// An order set of `EntityFlag`s used to describe certain Off-ledger
    /// user state about this Persona, e.g. if it is marked as hidden or not.
    #[serde(default)]
    pub flags: EntityFlags,

    pub persona_data: PersonaData,
}

impl Persona {
    /// Creates a new [`Persona`].
    pub fn new(
        persona_creating_factor_instance: HDFactorInstanceIdentityCreation,
        display_name: DisplayName,
        persona_data: PersonaData,
    ) -> Self {
        let address = IdentityAddress::from_hd_factor_instance_virtual_entity_creation(
            persona_creating_factor_instance.clone(),
        );
        Self {
            network_id: persona_creating_factor_instance
                .network_id()
                .into(),
            address,
            display_name,
            security_state: UnsecuredEntityControl::with_entity_creating_factor_instance(
                persona_creating_factor_instance,
            )
            .into(),
            flags: EntityFlags::default().into(),
            persona_data,
        }
    }

    /// Instantiates an persona with a display name, address and appearance id.
    pub fn placeholder_with_values(address: IdentityAddress, display_name: DisplayName) -> Self {
        Self {
            network_id: address.network_id.clone(),
            address,
            display_name,
            flags: EntityFlags::default().into(),
            security_state: EntitySecurityState::placeholder(),
            persona_data: PersonaData::default(),
        }
    }

    fn placeholder_at_index_name(index: HDPathValue, name: &str) -> Self {
        Self::placeholder_at_index_name_network(NetworkID::Mainnet, index, name)
    }

    fn placeholder_at_index_name_network(
        network_id: NetworkID,
        index: HDPathValue,
        name: &str,
    ) -> Self {
        let mwp = MnemonicWithPassphrase::placeholder();
        let bdfs = DeviceFactorSource::babylon(true, mwp.clone(), WalletClientModel::Iphone);
        let private_hd_factor_source = PrivateHierarchicalDeterministicFactorSource::new(mwp, bdfs);
        let persona_creating_factor_instance: HDFactorInstanceTransactionSigning<IdentityPath> =
            private_hd_factor_source.derive_entity_creation_factor_instance(network_id, index);

        Self::new(
            persona_creating_factor_instance,
            DisplayName::new(name).unwrap(),
            PersonaData::default(),
        )
    }

    /// A `Mainnet` persona named "Alice", a placeholder used to facilitate unit tests, with
    /// derivation index 0,
    pub fn placeholder_mainnet_alice() -> Self {
        Self::placeholder_at_index_name(0, "Alice")
    }

    /// A `Mainnet` persona named "Bob", a placeholder used to facilitate unit tests, with
    /// derivation index 1.
    pub fn placeholder_mainnet_bob() -> Self {
        Self::placeholder_at_index_name(1, "Bob")
    }

    /// A `Mainnet` persona named "Carol", a placeholder used to facilitate unit tests, with
    /// derivation index 2.
    pub fn placeholder_mainnet_carol() -> Self {
        Self::placeholder_at_index_name(2, "Carol")
    }

    /// A `Mainnet` persona named "Alice", a placeholder used to facilitate unit tests, with
    /// derivation index 0,
    pub fn placeholder_alice() -> Self {
        Self::placeholder_mainnet_alice()
    }

    /// A `Mainnet` persona named "Bob", a placeholder used to facilitate unit tests, with
    /// derivation index 1.
    pub fn placeholder_bob() -> Self {
        Self::placeholder_mainnet_bob()
    }

    /// A placeholder used to facilitate unit tests.
    pub fn placeholder_mainnet() -> Self {
        Self::placeholder_mainnet_alice()
    }

    /// A placeholder used to facilitate unit tests.
    pub fn placeholder_stokenet_carol() -> Self {
        Self::placeholder_at_index_name_network(NetworkID::Stokenet, 0, "Carol")
    }

    /// A placeholder used to facilitate unit tests.
    pub fn placeholder_stokenet() -> Self {
        Self::placeholder_stokenet_carol()
    }

    /// A `Mainnet` persona named "Batman", a placeholder used to facilitate unit tests,
    pub fn placeholder_persona_batman() -> Self {
        let identity_address: IdentityAddress =
            "identity_rdx12gzxlgre0glhh9jxaptm7tdth8j4w4r8ykpg2xjfv45nghzsjzrvmp"
                .try_into()
                .unwrap();
        Self::placeholder_with_values(identity_address, DisplayName::new("Batman").unwrap())
    }
}

impl Display for Persona {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} | {}", self.display_name, self.address)
    }
}

impl Ord for Persona {
    fn cmp(&self, other: &Self) -> Ordering {
        match (&self.security_state, &other.security_state) {
            (
                EntitySecurityState::Unsecured { value: l },
                EntitySecurityState::Unsecured { value: r },
            ) => l
                .transaction_signing
                .derivation_path()
                .last_component()
                .cmp(
                    r.transaction_signing
                        .derivation_path()
                        .last_component(),
                ),
        }
    }
}

impl PartialOrd for Persona {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

/// Add conformance to Identifiable in order to use `identified_vec`
impl Identifiable for Persona {
    type ID = IdentityAddress;

    fn id(&self) -> Self::ID {
        self.address.clone()
    }
}

/// Placeholder conformance to facilitate unit-tests
impl HasPlaceholder for Persona {
    fn placeholder() -> Self {
        Self::placeholder_alice()
    }

    fn placeholder_other() -> Self {
        Self::placeholder_bob()
    }
}

/// Empty struct to act as placeholder for PersonaData, `todo`
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Hash, Eq, Default, uniffi::Record)]
pub struct PersonaData {}

#[cfg(test)]
mod tests {
    use crate::{
        prelude::*,
        v100::entity::persona::{Persona, PersonaData},
    };
    use identified_vec::Identifiable;
    use std::str::FromStr;

    #[test]
    fn equality() {
        assert_eq!(Persona::placeholder(), Persona::placeholder());
        assert_eq!(Persona::placeholder_other(), Persona::placeholder_other());
    }

    #[test]
    fn inequality() {
        assert_ne!(Persona::placeholder(), Persona::placeholder_other());
    }

    #[test]
    fn compare() {
        assert!(Persona::placeholder_alice() < Persona::placeholder_bob());
    }

    #[test]
    fn new_with_identity_and_name() {
        let identity_address: IdentityAddress =
            "identity_rdx12gzxlgre0glhh9jxaptm7tdth8j4w4r8ykpg2xjfv45nghzsjzrvmp"
                .try_into()
                .unwrap();
        let persona = Persona::placeholder_with_values(
            identity_address.clone(),
            DisplayName::new("Batman").unwrap(),
        );
        assert_eq!(persona.address, identity_address);
    }

    #[test]
    fn display() {
        let account = Persona::placeholder_persona_batman();
        assert_eq!(
            format!("{account}"),
            "Batman | identity_rdx12gzxlgre0glhh9jxaptm7tdth8j4w4r8ykpg2xjfv45nghzsjzrvmp"
        );
    }

    #[test]
    fn display_name_get_set() {
        let mut persona = Persona::placeholder_persona_batman();
        assert_eq!(persona.display_name.value, "Batman");
        let new_display_name = DisplayName::new("Superman").unwrap();
        persona.display_name = new_display_name.clone();
        assert_eq!(persona.display_name, new_display_name);
    }

    #[test]
    fn update() {
        let mut persona = Persona::placeholder_persona_batman();
        assert_eq!(persona.display_name.value, "Batman");
        persona.display_name = DisplayName::new("Satoshi").unwrap();
        assert_eq!(persona.display_name.value, "Satoshi");
    }

    #[test]
    fn flags_get_set() {
        let mut persona = Persona::placeholder_persona_batman();
        assert_eq!(persona.flags, EntityFlags::default());
        let new_flags = EntityFlags::with_flag(EntityFlag::DeletedByUser);
        persona.flags = new_flags.clone();
        assert_eq!(persona.flags, new_flags);
    }

    #[test]
    fn json_serialize() {
        let model = Persona::placeholder_alice();
        let jsonstr = serde_json::to_string(&model).unwrap();
        println!("{}", jsonstr);
    }

    #[test]
    fn placerholder_display_name() {
        let placeholder = Persona::placeholder();
        assert_eq!(placeholder.display_name, DisplayName::new("Alice").unwrap());
    }

    #[test]
    fn placeholder_other_display_name() {
        let placeholder_other = Persona::placeholder_other();
        assert_eq!(
            placeholder_other.display_name,
            DisplayName::new("Bob").unwrap()
        );
    }

    #[test]
    fn placeholder_alice_security_state() {
        let persona = Persona::placeholder_alice();
        let jsonstr = serde_json::to_string(&persona.security_state).unwrap();
        println!("{:#?}", jsonstr);
        // assert_eq!(persona.security_state, EntitySecurityState::Unsecured {  });
    }

    #[test]
    fn identifiable() {
        let persona = Persona::placeholder_persona_batman();
        let identity_address: IdentityAddress =
            "identity_rdx12gzxlgre0glhh9jxaptm7tdth8j4w4r8ykpg2xjfv45nghzsjzrvmp"
                .try_into()
                .unwrap();
        assert_eq!(persona.id(), identity_address);
    }

    #[test]
    fn json_roundtrip_batman() {
        let model = Persona::placeholder_persona_batman();
        assert_eq_after_json_roundtrip(
            &model,
            r#"
            {
                "networkID": 1,
                "address": "identity_rdx12gzxlgre0glhh9jxaptm7tdth8j4w4r8ykpg2xjfv45nghzsjzrvmp",
                "displayName": "Batman",
                "securityState": {
                  "discriminator": "unsecured",
                  "unsecuredEntityControl": {
                    "transactionSigning": {
                      "factorSourceID": {
                        "discriminator": "fromHash",
                        "fromHash": {
                          "kind": "device",
                          "body": "3c986ebf9dcd9167a97036d3b2c997433e85e6cc4e4422ad89269dac7bfea240"
                        }
                      },
                      "badge": {
                        "discriminator": "virtualSource",
                        "virtualSource": {
                          "discriminator": "hierarchicalDeterministicPublicKey",
                          "hierarchicalDeterministicPublicKey": {
                            "publicKey": {
                              "curve": "curve25519",
                              "compressedData": "d24cc6af91c3f103d7f46e5691ce2af9fea7d90cfb89a89d5bba4b513b34be3b"
                            },
                            "derivationPath": {
                              "scheme": "cap26",
                              "path": "m/44H/1022H/1H/525H/1460H/0H"
                            }
                          }
                        }
                      }
                    }
                  }
                },
                "flags": [],
                "personaData": {}
              }
            "#,
        );
    }

    #[test]
    fn json_deserialization_works_without_flags_as_version_1_0_0_of_app() {
        let json = serde_json::Value::from_str(
            r#"
            {
                "networkID": 1,
                "address": "identity_rdx12gzxlgre0glhh9jxaptm7tdth8j4w4r8ykpg2xjfv45nghzsjzrvmp",
                "displayName": "Batman",
                "securityState": {
                  "discriminator": "unsecured",
                  "unsecuredEntityControl": {
                    "transactionSigning": {
                      "factorSourceID": {
                        "discriminator": "fromHash",
                        "fromHash": {
                          "kind": "device",
                          "body": "3c986ebf9dcd9167a97036d3b2c997433e85e6cc4e4422ad89269dac7bfea240"
                        }
                      },
                      "badge": {
                        "discriminator": "virtualSource",
                        "virtualSource": {
                          "discriminator": "hierarchicalDeterministicPublicKey",
                          "hierarchicalDeterministicPublicKey": {
                            "publicKey": {
                              "curve": "curve25519",
                              "compressedData": "d24cc6af91c3f103d7f46e5691ce2af9fea7d90cfb89a89d5bba4b513b34be3b"
                            },
                            "derivationPath": {
                              "scheme": "cap26",
                              "path": "m/44H/1022H/1H/525H/1460H/0H"
                            }
                          }
                        }
                      }
                    }
                  }
                },
                "flags": [],
                "personaData": {}
              }
            "#,
        ).unwrap();
        let persona = serde_json::from_value::<Persona>(json).unwrap();
        assert_eq!(persona.display_name.value, "Batman".to_string()); // soundness
        assert_eq!(persona.flags.len(), 0); // assert Default value is empty flags.
        assert_eq!(persona.persona_data, PersonaData {}); // assert Default value is empty flags.
    }

    #[test]
    fn placeholder_mainnet() {
        let persona = Persona::placeholder_mainnet();
        assert_eq!(persona.display_name.value, "Alice".to_string());
        assert_eq!(persona.network_id, NetworkID::Mainnet);
    }

    #[test]
    fn placeholder_stokenet() {
        let persona = Persona::placeholder_stokenet();
        assert_eq!(persona.display_name.value, "Carol".to_string());
        assert_eq!(persona.network_id, NetworkID::Stokenet);
    }

    #[test]
    fn placeholder_stokenet_carol() {
        let persona = Persona::placeholder_stokenet_carol();
        assert_eq!(persona.display_name.value, "Carol".to_string());
        assert_eq!(persona.network_id, NetworkID::Stokenet);
    }

    #[test]
    fn placeholder_mainnet_carol() {
        let persona = Persona::placeholder_mainnet_carol();
        assert_eq!(persona.display_name.value, "Carol".to_string());
        assert_eq!(persona.network_id, NetworkID::Mainnet);
    }
}
