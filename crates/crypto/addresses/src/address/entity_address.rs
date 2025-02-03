use crate::prelude::*;

use radix_engine_toolkit::functions::derive::{
    preallocated_account_address_from_public_key as RET_new_account_address,
    preallocated_identity_address_from_public_key as RET_new_identity_address,
};

/// An address of an entity, provides default implementation of `try_from_bech32`
/// to decode a bech32 encoded address string into Self.
pub trait IsBaseEntityAddress:
    HasEntityKindObjectSafe
    + Into<AddressOfAccountOrPersona>
    + Clone
    + IsNetworkAware
{
}

pub trait IsEntityAddress:
    IsBaseEntityAddress
    + HasEntityKind
    + AddressFromNodeId
    + std::hash::Hash
    + std::cmp::Eq
{
    /// Creates a new address from `public_key` and `network_id` by bech32 encoding
    /// it.
    fn from_public_key<P>(public_key: P, network_id: NetworkID) -> Self
    where
        P: Into<ScryptoPublicKey> + Clone,
    {
        let component = match Self::entity_kind() {
            CAP26EntityKind::Account => RET_new_account_address(&public_key),
            CAP26EntityKind::Identity => RET_new_identity_address(&public_key),
        };

        let node_id = component.into_node_id();

        Self::new_from_node_id(node_id, network_id).expect("To always be able to create a address from public key and network id.")
    }

    fn from_hd_factor_instance_virtual_entity_creation<E: IsEntityPath>(
        hd_factor_instance_virtual_entity_creation: HDFactorInstanceTransactionSigning<E>,
    ) -> Self {
        let network_id =
            hd_factor_instance_virtual_entity_creation.path.network_id();

        Self::from_public_key(
            hd_factor_instance_virtual_entity_creation
                .public_key()
                .public_key,
            network_id,
        )
    }
}
