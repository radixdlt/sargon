use crate::prelude::*;

pub trait SamplesFromIdentityAddress {
    fn sample_from_identity_address(identity_address: IdentityAddress) -> Self;
}

impl SamplesFromIdentityAddress for AddressesOfAccessController {
    fn sample_from_identity_address(identity_address: IdentityAddress) -> Self {
        let node_id: [u8; 29] = identity_address.node_id().as_bytes()[0..29]
            .try_into()
            .unwrap();

        let access_controller_address =
            AccessControllerAddress::with_node_id_bytes(
                &node_id,
                identity_address.network_id(),
            );

        let xrd_vault_address = VaultAddress::with_node_id_bytes(
            &node_id,
            identity_address.network_id(),
        );

        Self::new(access_controller_address, xrd_vault_address)
    }
}
