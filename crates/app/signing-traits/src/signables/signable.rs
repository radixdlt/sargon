use crate::prelude::*;

/// Any type conforming to `Signable` can be used with `SignaturesCollector` and collect
/// signatures from all involved entities according to their security structure.
pub trait Signable:
    std::hash::Hash + PartialEq + Eq + Clone + Debug + Send + Sync
{
    /// A stable identifier for this `Signable`.
    type ID: SignableID;

    /// A compiled version of the `Signable` that is passed down to the interactors.
    type Payload: PartialEq
        + Eq
        + Clone
        + Debug
        + std::hash::Hash
        + Into<Self::ID>
        + From<Self>
        + Send
        + Sync;

    type Signed: Clone
        + Debug
        + Into<Self>
        + IntoIterator<Item = SignatureWithPublicKey>;

    /// A function that extracts the involved entities that require signing.
    fn entities_requiring_signing(
        &self,
        entity_querying: &impl GetEntityByAddress,
    ) -> Result<IndexSet<AccountOrPersona>>;

    fn get_payload(&self) -> Self::Payload {
        From::<Self>::from(self.clone())
    }

    /// Retrieves the stable identifier from the `Signable`
    fn get_id(&self) -> Self::ID {
        self.get_payload().into()
    }

    fn signed(
        &self,
        signatures: IndexSet<HDSignature<Self::ID>>,
    ) -> Result<Self::Signed>;
}

/// An identifier that is unique for each `Signable`
pub trait SignableID:
    Eq + StdHash + Clone + Debug + Into<Hash> + Send + Sync
{
}

impl SignableID for Exactly32Bytes {}

/// A trait which provides the ability to construct a `Signable` sample by building a manifest.
pub trait ProvidesSamplesByBuildingManifest:
    PartialEq + Eq + Clone + Send + Sync
{
    /// Returns a sample `Signable` that its summary will involve all the
    /// `accounts_requiring_auth` and `personas_requiring_auth` in entities requiring auth.
    /// This can be accomplished by building a manifest that constructs owner keys from these
    /// entities. All entities set the same `PublicKeyHash` for the sake of simplicity.
    fn sample_entities_requiring_auth<'a, 'p>(
        accounts_requiring_auth: impl IntoIterator<Item = &'a Account>,
        personas_requiring_auth: impl IntoIterator<Item = &'p Persona>,
    ) -> Self {
        Self::sample_entity_addresses_requiring_auth(
            accounts_requiring_auth.into_iter().map(|a| a.address),
            personas_requiring_auth.into_iter().map(|p| p.address),
        )
    }

    /// Returns a sample `Signable` that its summary will involve all the
    /// `account_addresses_requiring_auth` and `identity_addresses_requiring_auth` in
    /// entities requiring auth.
    /// This can be accomplished by building a manifest that constructs owner keys from these
    /// entity addresses. All entities set the same `PublicKeyHash` for the sake of simplicity.
    fn sample_entity_addresses_requiring_auth(
        account_addresses_requiring_auth: impl IntoIterator<Item = AccountAddress>,
        identity_addresses_requiring_auth: impl IntoIterator<Item = IdentityAddress>,
    ) -> Self {
        Self::sample_entity_addresses_with_pub_key_hashes_requiring_auth(
            account_addresses_requiring_auth
                .into_iter()
                .map(|a| (a, PublicKeyHash::sample())),
            identity_addresses_requiring_auth
                .into_iter()
                .map(|a| (a, PublicKeyHash::sample())),
        )
    }

    fn sample_entity_addresses_with_pub_key_hashes_requiring_auth(
        account_addresses_requiring_auth: impl IntoIterator<
            Item = (AccountAddress, PublicKeyHash),
        >,
        identity_addresses_requiring_auth: impl IntoIterator<
            Item = (IdentityAddress, PublicKeyHash),
        >,
    ) -> Self {
        let mut network_id: Option<NetworkID> = None;

        let all_addresses_with_hashes = account_addresses_requiring_auth
            .into_iter()
            .map(|(address, hash)| {
                (AddressOfAccountOrPersona::from(address), hash)
            })
            .chain(identity_addresses_requiring_auth.into_iter().map(
                |(address, hash)| {
                    (AddressOfAccountOrPersona::from(address), hash)
                },
            ))
            .collect::<Vec<_>>();

        all_addresses_with_hashes
            .iter()
            .for_each(|(address, _hash)| {
                if let Some(network_id) = network_id {
                    assert_eq!(network_id, address.network_id())
                } else {
                    network_id = Some(address.network_id())
                }
            });

        Self::sample_entity_addresses_with_pub_key_hashes(
            all_addresses_with_hashes,
            network_id,
        )
    }

    fn sample_entity_addresses_with_pub_key_hashes(
        all_addresses_with_hashes: Vec<(
            AddressOfAccountOrPersona,
            PublicKeyHash,
        )>,
        network_id: Option<NetworkID>,
    ) -> Self;
}
