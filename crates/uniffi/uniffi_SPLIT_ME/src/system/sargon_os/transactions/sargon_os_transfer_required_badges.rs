use sargon::OsTransferRequiredBadges;

use crate::prelude::*;

// ==================
// Transfer required badges (Public)
// ==================
#[uniffi::export]
impl SargonOS {
    /// What withdrawing `resources` from `from_account` needs: the badges to prove (give them to
    /// `manifest_per_asset_transfers` / `manifest_per_recipient_transfers`), and the restricted
    /// resources with what their withdraw rules ask for (to show to the person).
    pub async fn transfer_required_badges(
        &self,
        from_account: AccountAddress,
        resources: Vec<ResourceAddress>,
    ) -> Result<TransferRequiredBadges> {
        self.wrapped
            .transfer_required_badges(
                from_account.into_internal(),
                resources.into_internal(),
            )
            .await
            .into_result()
    }
}
