use crate::prelude::*;
use sargon::{
    EntitiesAffectedWithBdfsError as InternalEntitiesAffectedWithBdfsError,
    GetEntitiesAffectedByBdfsError,
};

#[uniffi::export]
impl SargonOS {
    /// There used to be a bug on Android hosts that entities created with
    /// 1. an olympia factor source but
    /// 2. their hd public keys were using the Ed25519 curve
    ///
    /// The users affected by this bug were prompted to contact support from
    /// the [1.0.5](https://github.com/radixdlt/babylon-wallet-android/releases/tag/1.0.5) version.
    ///
    /// **See also**
    /// Related PR that detects the issue: [#533](https://github.com/radixdlt/babylon-wallet-android/pull/533)
    /// Later PR that reports which entities are affected: [#897](https://github.com/radixdlt/babylon-wallet-android/pull/897)
    async fn get_entities_affected_by_bdfs_error(
        &self,
    ) -> Option<EntitiesAffectedByBdfsError> {
        self.wrapped
            .get_entities_affected_by_bdfs_error()
            .await
            .map(|e| e.into())
    }
}

#[derive(Debug, Clone, uniffi::Record)]
pub struct EntitiesAffectedByBdfsError {
    pub affected_account_addresses: Vec<AccountAddress>,
    pub affected_identity_addresses: Vec<IdentityAddress>,
}

impl EntitiesAffectedByBdfsError {
    pub fn into_internal(&self) -> InternalEntitiesAffectedWithBdfsError {
        self.clone().into()
    }
}

impl From<InternalEntitiesAffectedWithBdfsError>
    for EntitiesAffectedByBdfsError
{
    fn from(value: InternalEntitiesAffectedWithBdfsError) -> Self {
        Self {
            affected_account_addresses: value
                .affected_account_addresses
                .into_iter()
                .map(|a| a.into())
                .collect_vec(),
            affected_identity_addresses: value
                .affected_identity_addresses
                .into_iter()
                .map(|a| a.into())
                .collect_vec(),
        }
    }
}

impl From<EntitiesAffectedByBdfsError>
    for InternalEntitiesAffectedWithBdfsError
{
    fn from(value: EntitiesAffectedByBdfsError) -> Self {
        Self {
            affected_account_addresses: IndexSet::from_iter(
                value
                    .affected_account_addresses
                    .into_iter()
                    .map(|a| a.into()),
            ),
            affected_identity_addresses: IndexSet::from_iter(
                value
                    .affected_identity_addresses
                    .into_iter()
                    .map(|a| a.into()),
            ),
        }
    }
}
