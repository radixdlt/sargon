pub mod signable_intents_builder;
pub mod signature_collector_factory;
pub mod signatures_collector_orchestrator;

pub use signable_intents_builder::*;
pub use signature_collector_factory::*;
pub use signatures_collector_orchestrator::*;

use crate::prelude::*;

pub async fn sign_access_controller_stop_timed_recovery_transaction(
    os: &SargonOS,
    base_transaction_intent: TransactionIntent,
    ac_address: AccessControllerAddress,
) -> Result<SignedIntent> {
    let profile = os.profile()?;
    let gw_client = os.gateway_client();

    let fee_payer = AccessControllerTransactionFeePayer::of(
        &base_transaction_intent,
        &profile,
    )?;
    let fee_payer_account = fee_payer
        .account()
        .map(|address| profile.account_by_address(address))
        .transpose()?;

    let ac_state_details = gw_client
        .fetch_access_controller_details(ac_address)
        .await?;

    let securified_entity =
        profile.entity_by_access_controller_address(ac_address)?;

    let factory = StopTimedRecoverySignaturesCollectorFactory::new(
        base_transaction_intent,
        AnySecurifiedEntity::new(securified_entity)?,
        fee_payer,
        fee_payer_account,
        ac_state_details,
        Arc::new(DefaultTransactionIntentSignaturesCollectorBuilder::new(
            os.sign_transactions_interactor(),
            IndexSet::from_iter(profile.factor_sources.iter()),
        )),
    )?;

    StopTimedRecoverySignaturesCollectorOrchestrator::new(factory)
        .sign()
        .await
}
