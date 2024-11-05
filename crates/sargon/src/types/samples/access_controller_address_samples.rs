use crate::{
    AccessControllerAddress, AccountAddress, HasNodeId, IdentityAddress, IsNetworkAware
};

impl AccessControllerAddress {
    pub fn sample_from_account_address(
        account_address: AccountAddress,
    ) -> Self {
        let node_id: [u8; 29] = account_address.node_id().as_bytes()[0..29]
            .try_into()
            .unwrap();

        AccessControllerAddress::with_node_id_bytes(
            &node_id,
            account_address.network_id(),
        )
    }

    pub fn sample_from_identity_address(
        identity_address: IdentityAddress,
    ) -> Self {
        let node_id: [u8; 29] = identity_address.node_id().as_bytes()[0..29]
            .try_into()
            .unwrap();

        AccessControllerAddress::with_node_id_bytes(
            &node_id,
            identity_address.network_id(),
        )
    }
}
