use crate::prelude::*;
use profile_logic::EntityUnsecuredControllingFactorInstance;
use sargon::Persona as InternalPersona;

decl_vec_samples_for!(Personas, Persona);

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
#[derive(Clone, PartialEq, Hash, Eq, uniffi::Record)]
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
    pub flags: Vec<EntityFlag>,

    /// Personal information a user has associated with a certain Persona, of different kinds, such as name,
    /// email address(es) or phone number(s). This information is only ever stored in Profile and is never
    /// uploaded to the Radix Network.
    pub persona_data: PersonaData,
}

impl Persona {
    pub fn into_internal(&self) -> InternalPersona {
        self.clone().into()
    }
}

impl From<Persona> for InternalPersona {
    fn from(value: Persona) -> Self {
        Self::with(
            value.network_id,
            value.address,
            value.display_name,
            value.security_state,
            value.flags.into_iter().map(Into::into),
            value.persona_data,
        )
    }
}

impl From<InternalPersona> for Persona {
    fn from(value: InternalPersona) -> Self {
        Self {
            network_id: value.network_id.into(),
            address: value.address.into(),
            display_name: value.display_name.into(),
            security_state: value.security_state.clone().into(),
            flags: value.flags.clone().into_iter().map(Into::into).collect(),
            persona_data: value.persona_data.into(),
        }
    }
}

#[uniffi::export]
pub fn new_persona_sample() -> Persona {
    InternalPersona::sample().into()
}

#[uniffi::export]
pub fn new_persona_sample_other() -> Persona {
    InternalPersona::sample_other().into()
}

#[uniffi::export]
pub fn new_persona_sample_mainnet_batman() -> Persona {
    InternalPersona::sample_mainnet_batman().into()
}

#[uniffi::export]
pub fn new_persona_sample_mainnet_satoshi() -> Persona {
    InternalPersona::sample_mainnet_satoshi().into()
}

#[uniffi::export]
pub fn new_persona_sample_mainnet_ripley() -> Persona {
    InternalPersona::sample_mainnet_ripley().into()
}

#[uniffi::export]
pub fn new_persona_sample_mainnet_turing() -> Persona {
    InternalPersona::sample_mainnet_turing().into()
}

#[uniffi::export]
pub fn new_persona_sample_stokenet_leia_skywalker() -> Persona {
    InternalPersona::sample_stokenet_leia_skywalker().into()
}

#[uniffi::export]
pub fn new_persona_sample_stokenet_hermione() -> Persona {
    InternalPersona::sample_stokenet_hermione().into()
}

#[uniffi::export]
pub fn new_persona_sample_stokenet_connor() -> Persona {
    InternalPersona::sample_stokenet_connor().into()
}

#[uniffi::export]
pub fn persona_unsecured_controlling_factor_instance(
    persona: Persona,
) -> Option<HierarchicalDeterministicFactorInstance> {
    persona
        .into_internal()
        .unsecured_controlling_factor_instance()
        .map(|key| key.into())
}
