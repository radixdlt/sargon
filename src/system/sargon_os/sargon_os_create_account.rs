use crate::prelude::*;

impl SargonOS {
    /// Creates a new non securified account **WITHOUT** add it to Profile, using the *main* "Babylon"
    /// `DeviceFactorSource` and the "next" index for this FactorSource as derivation path.
    ///
    /// If you want to add it to Profile, call `wallet.add_account(account)`
    pub async fn create_unsaved_account(
        &self,
        network_id: NetworkID,
        name: DisplayName,
    ) -> Result<Account> {
        let profile = self.profile_holder.access_profile_with(|p| p.clone());
        profile
            .create_unsaved_account(network_id, name, async move |fs| {
                self.load_private_device_factor_source(&fs).await
            })
            .await
    }
}
