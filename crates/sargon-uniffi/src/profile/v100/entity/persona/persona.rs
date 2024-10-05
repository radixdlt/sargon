use crate::prelude::*;

/// A Persona is an identity a user chooses to login to a dApp with, using
/// RadixConnect - Radix decentralized login solution. A persona is very
/// similar to [`Account`]s, in the sense that they are On-Network/On-Ledger
/// components, with a unique network dependent address ([`IdentityAddress`])
/// and with a security state (see [`EntitySecurityState`]) knowing which
/// factor instances that control this component, but with one important
/// difference: a Persona cannot hold funds. It is impossible to transfer
/// any asset to a Persona. The On-Network component representation of
/// the Persona is called `Identity`. The concept "Persona" is a Radix
/// Wallet (Profile) *application* of an Identity.
///
/// Personas have data (see [`PersonaData`]), which is personal information
/// a user has associated with a this Persona, of different kinds, such as name,
/// email address(es) or phone number(s). The `PersonaData` is **never** uploaded
/// to the Radix Network, i.e. it is a pure Radix Wallet (Profile) construct,
/// On-Network Identities does not know of PersonaData, and never will (well
/// technically, nothing stops a user from building their own wallet and uploading
/// personal information to the metadata of the Identity component... but `Sargon`
/// never will, nor will the Radix Wallet.).
#[derive(
    Clone,
    Debug,
    PartialEq,
    Hash,
    Eq,
    derive_more::Display,
    uniffi::Record,
)]
#[display("{} | {}", display_name, address)]
pub struct Persona {
    /// The ID of the network this account can be used with.
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
    pub flags: EntityFlags,

    /// Personal information a user has associated with a certain Persona, of different kinds, such as name,
    /// email address(es) or phone number(s). This information is only ever stored in Profile and is never
    /// uploaded to the Radix Network.
    pub persona_data: PersonaData,
}

/// Add conformance to Identifiable in order to use `IdentifiedVecOf`
impl Identifiable for Persona {
    type ID = IdentityAddress;

    fn id(&self) -> Self::ID {
        self.address
    }
}

#[uniffi::export]
pub fn new_persona_sample() -> Persona {
    Persona::sample()
}

#[uniffi::export]
pub fn new_persona_sample_other() -> Persona {
    Persona::sample_other()
}

#[uniffi::export]
pub fn new_persona_sample_mainnet_batman() -> Persona {
    Persona::sample_mainnet_batman()
}

#[uniffi::export]
pub fn new_persona_sample_mainnet_satoshi() -> Persona {
    Persona::sample_mainnet_satoshi()
}

#[uniffi::export]
pub fn new_persona_sample_mainnet_ripley() -> Persona {
    Persona::sample_mainnet_ripley()
}

#[uniffi::export]
pub fn new_persona_sample_mainnet_turing() -> Persona {
    Persona::sample_mainnet_turing()
}

#[uniffi::export]
pub fn new_persona_sample_stokenet_leia_skywalker() -> Persona {
    Persona::sample_stokenet_leia_skywalker()
}

#[uniffi::export]
pub fn new_persona_sample_stokenet_hermione() -> Persona {
    Persona::sample_stokenet_hermione()
}

#[uniffi::export]
pub fn new_persona_sample_stokenet_connor() -> Persona {
    Persona::sample_stokenet_connor()
}

#[cfg(test)]
mod uniffi_tests {
    use crate::prelude::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = Persona;

    #[test]
    fn samples() {
        assert_eq!(new_persona_sample(), SUT::sample());

        assert_eq!(new_persona_sample_other(), SUT::sample_other());

        assert_eq!(
            new_persona_sample_mainnet_batman(),
            SUT::sample_mainnet_batman()
        );

        assert_eq!(
            new_persona_sample_mainnet_satoshi(),
            SUT::sample_mainnet_satoshi()
        );

        assert_eq!(
            new_persona_sample_stokenet_leia_skywalker(),
            SUT::sample_stokenet_leia_skywalker()
        );

        assert_eq!(
            new_persona_sample_stokenet_hermione(),
            SUT::sample_stokenet_hermione()
        );
    }

    #[test]
    fn hash_of_sample_values() {
        assert_eq!(
            HashSet::<SUT>::from_iter([
                new_persona_sample_mainnet_batman(),
                new_persona_sample_mainnet_satoshi(),
                new_persona_sample_mainnet_ripley(),
                new_persona_sample_mainnet_turing(),
                new_persona_sample_stokenet_hermione(),
                new_persona_sample_stokenet_leia_skywalker(),
                new_persona_sample_stokenet_connor(),
                // duplicates should be removed
                new_persona_sample_mainnet_batman(),
                new_persona_sample_mainnet_satoshi(),
                new_persona_sample_mainnet_ripley(),
                new_persona_sample_mainnet_turing(),
                new_persona_sample_stokenet_hermione(),
                new_persona_sample_stokenet_leia_skywalker(),
                new_persona_sample_stokenet_connor(),
            ])
            .len(),
            7
        )
    }
}
