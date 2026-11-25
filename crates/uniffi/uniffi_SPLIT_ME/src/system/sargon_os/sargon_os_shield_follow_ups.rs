use crate::prelude::*;

#[uniffi::export]
impl SargonOS {
    /// A transaction that sets the ROLA key of the securified entity at
    /// `address` to the one of its security structure. Make it once the new
    /// security structure is in place: securifying an entity or updating its
    /// shield doesn't set the ROLA key.
    pub async fn make_set_rola_key_manifest(
        &self,
        address: AddressOfAccountOrPersona,
    ) -> Result<TransactionManifest> {
        self.wrapped
            .make_set_rola_key_manifest(address.into())
            .await
            .into_result()
    }

    /// A transaction that tops up the recovery fee vault of the access
    /// controller of the securified entity at `address` with `amount` XRD
    /// from the account at `payer`. Securifying an entity or updating its
    /// shield doesn't top up the vault.
    pub async fn make_top_up_access_controller_xrd_vault_manifest(
        &self,
        address: AddressOfAccountOrPersona,
        payer: AccountAddress,
        amount: Decimal192,
    ) -> Result<TransactionManifest> {
        self.wrapped
            .make_top_up_access_controller_xrd_vault_manifest(
                address.into(),
                payer.into(),
                amount.into(),
            )
            .await
            .into_result()
    }
}
