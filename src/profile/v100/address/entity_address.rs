use crate::prelude::*;
use radix_engine_common::{
    address::AddressBech32Encoder, crypto::PublicKey as ScryptoPublicKey,
    types::NodeId as ScryptoNodeId,
};
use radix_engine_toolkit::functions::derive::{
    virtual_account_address_from_public_key as RET_new_account_address,
    virtual_identity_address_from_public_key as RET_new_identity_address,
};

/// An address of an entity, provides default implementation of `try_from_bech32`
/// to decode a bech32 encoded address string into Self.
pub trait EntityAddress: Sized {
    fn entity_type() -> AbstractEntityType;

    // Underscored to decrease visibility. You SHOULD NOT call this function directly,
    // instead use `try_from_bech32` which performs proper validation. Impl types SHOULD
    // `panic` if `address` does not start with `Self::entity_type().hrp()`
    fn __with_address_and_network_id(
        address: &str,
        network_id: NetworkID,
    ) -> Self;

    fn address_from_node_id(
        node_id: ScryptoNodeId,
        network_id: NetworkID,
    ) -> String {
        let bech32_encoder =
            AddressBech32Encoder::new(&network_id.network_definition());
        bech32_encoder
            .encode(node_id.as_bytes())
            .expect("Should always be able to format address")
    }

    /// Creates a new address from `public_key` and `network_id` by bech32 encoding
    /// it.
    #[cfg(not(tarpaulin_include))] // false negative
    fn from_public_key<P>(public_key: P, network_id: NetworkID) -> Self
    where
        P: Into<ScryptoPublicKey> + Clone,
    {
        let component = match Self::entity_type() {
            AbstractEntityType::Account => RET_new_account_address(&public_key),
            AbstractEntityType::Identity => {
                RET_new_identity_address(&public_key)
            }
            AbstractEntityType::Resource => panic!("resource"),
        };

        let address =
            Self::address_from_node_id(component.into_node_id(), network_id);
        Self::__with_address_and_network_id(&address, network_id)
    }

    #[cfg(not(tarpaulin_include))] // false negative
    fn from_hd_factor_instance_virtual_entity_creation<
        E: IsEntityPath + Clone,
    >(
        hd_factor_instance_virtual_entity_creation: HDFactorInstanceTransactionSigning<E>,
    ) -> Self {
        let network_id =
            hd_factor_instance_virtual_entity_creation.path.network_id();

        Self::from_public_key(
            hd_factor_instance_virtual_entity_creation
                .public_key()
                .public_key
                .clone(),
            network_id,
        )
    }

    #[cfg(not(tarpaulin_include))] // false negative
    fn try_from_bech32(s: &str) -> Result<Self> {
        let (network_id, entity_type, hrp, _) = decode_address(s)?;
        if entity_type != Self::entity_type() {
            return Err(CommonError::MismatchingEntityTypeWhileDecodingAddress);
        }

        assert!(hrp.starts_with(&entity_type.hrp()), "Mismatching HRP while decoding address, this should never happen. Did internal function `decode_address` change? Or did you accidentally change or impl the `hrp` method on EntityType?");

        Ok(Self::__with_address_and_network_id(s, network_id))
    }
}
