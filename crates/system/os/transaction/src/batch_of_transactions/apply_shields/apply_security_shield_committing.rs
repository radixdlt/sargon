use crate::prelude::*;

#[async_trait::async_trait]
pub trait ApplySecurityShieldCommitting: Send + Sync {
    /// Host has previously called the function
    ///     `make_interaction_for_applying_security_shield`
    /// and specified the `security_shield_id` and `addresses` of the entities
    /// for which they want to apply the security shield. Which returns a Vec
    /// of TransactionManifests, one for each entity. If the entity is securified
    /// already the "variant" `RolesExercisableInTransactionManifestCombination::InitiateWithPrimaryCompleteWithRecovery` is used.
    ///
    /// Host presents batch TX review UI, and user needs to select payer for each manifest,
    /// MUST be done for Personas and in case of entity being an Account, the payer might
    /// be the same account as the entity applying the shield. That information is passed
    /// when user slides to sign back to Sargon via the tuples of `ManifestWithPayer`.
    ///
    /// We will map from `Vec<Manifest>` -> `Vec<Vec<Manifest>>` where for each entity
    /// being unsecurified the inner Vec will be unchanged - one single manifest. But
    /// for each securified entity - which has a manifest which was create with `InitiateWithPrimaryCompleteWithRecovery` variant, we will map to 4 manifests, where
    /// the three new manifests are created by specifying:
    /// - `InitiateWithPrimaryCompleteWithConfirmation`
    /// - `InitiateWithRecoveryCompleteWithConfirmation`
    /// - `InitiateWithRecoveryDelayedCompletion`
    ///
    /// Then we will inner map of the `Vec<Vec<Manifest>>` to
    /// perform look up of all `payer` address and get the Account from
    /// Profile - and depending on if that payer account is already securified or not
    /// we will use `modify_add_lock_fee` for Unsecurified accounts and for securified
    /// accounts we will use `modify_manifest_add_lock_fee_against_xrd_vault_of_access_controller`.
    ///
    /// Then we will build TransactionIntent for all of these - with broad enough
    /// an epoch window so that we can submit these with delay in between.
    ///
    /// We will compile them and we will start the process of signing them. Which will be the job of `SigningManager` - many instances of `SignaturesCollector` using one Role at a time.
    ///
    /// Can work with single transaction of course...
    async fn sign_and_enqueue_batch_of_transactions_applying_security_shield(
        &self,
        network_id: NetworkID,
        manifest_and_payer_tuples: IndexSet<ManifestWithPayerByAddress>,
    ) -> Result<IndexSet<TransactionIntentHash>>;
}

#[async_trait::async_trait]
impl ApplySecurityShieldCommitting for SargonOS {
    async fn sign_and_enqueue_batch_of_transactions_applying_security_shield(
        &self,
        network_id: NetworkID,
        manifest_and_payer_tuples: IndexSet<ManifestWithPayerByAddress>,
    ) -> Result<IndexSet<TransactionIntentHash>> {
        let committer = ApplyShieldTransactionsCommitterImpl::new(self)?;
        committer.commit(network_id, manifest_and_payer_tuples).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[actix_rt::test]
    async fn test() {
        let os = SargonOS::fast_boot().await;
        os.sign_and_enqueue_batch_of_transactions_applying_security_shield(
            NetworkID::Mainnet,
            IndexSet::new(),
        )
        .await;
    }
}
