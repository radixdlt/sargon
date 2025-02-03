use crate::prelude::*;

pub trait SamplesFromAccountAddress {
    fn sample_from_account_address(account_address: AccountAddress) -> Self;
}

impl SamplesFromAccountAddress for AddressesOfAccessController {
    fn sample_from_account_address(account_address: AccountAddress) -> Self {
        let node_id: [u8; 29] = account_address.node_id().as_bytes()[0..29]
            .try_into()
            .unwrap();

        let access_controller_address =
            AccessControllerAddress::with_node_id_bytes(
                &node_id,
                account_address.network_id(),
            );

        let xrd_vault_address = VaultAddress::with_node_id_bytes(
            &node_id,
            account_address.network_id(),
        );

        Self::new(access_controller_address, xrd_vault_address)
    }
}
