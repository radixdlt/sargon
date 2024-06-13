use crate::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq, Hash, uniffi::Record)]
pub struct HierarchicalDeterministicSignature {
    pub factor: HierarchicalDeterministicFactorInstance,
    pub signature: Signature,
}
impl Identifiable for HierarchicalDeterministicSignature {
    type ID = HierarchicalDeterministicFactorInstance;
    fn id(&self) -> Self::ID {
        self.factor.clone()
    }
}

#[derive(Clone, Debug, PartialEq, Eq, uniffi::Enum)]
pub enum PayloadToSign {
    Intent(IntentHash),
    ROLA(Hash),
}

#[async_trait::async_trait]
pub trait SignWithFactorSource<Factor: IsFactorSource> {
    async fn sign(
        &self,
        factor_source: Factor,
        derivation_paths: Vec<DerivationPath>,
        payload: PayloadToSign,
    ) -> Result<IdentifiedVecOf<HierarchicalDeterministicSignature>>;
}

#[uniffi::export(with_foreign)]
#[async_trait::async_trait]
pub trait UseDeviceFactorSourceDriver: Send + Sync + std::fmt::Debug {
    async fn sign_with_device(
        &self,
        factor_source: DeviceFactorSource,
        derivation_paths: Vec<DerivationPath>,
        payload: PayloadToSign,
    ) -> Result<IdentifiedVecOf<HierarchicalDeterministicSignature>>;
}

#[async_trait::async_trait]
impl<T: UseDeviceFactorSourceDriver> SignWithFactorSource<DeviceFactorSource>
    for T
{
    async fn sign(
        &self,
        factor_source: DeviceFactorSource,
        derivation_paths: Vec<DerivationPath>,
        payload: PayloadToSign,
    ) -> Result<IdentifiedVecOf<HierarchicalDeterministicSignature>> {
        self.sign_with_device(factor_source, derivation_paths, payload)
            .await
    }
}

#[derive(Debug)]
pub struct SigningClient {
    use_device_factor_source_driver: Arc<dyn UseDeviceFactorSourceDriver>,
}

impl SigningClient {
    pub(crate) fn new(
        use_device_factor_source_driver: Arc<dyn UseDeviceFactorSourceDriver>,
    ) -> Self {
        Self {
            use_device_factor_source_driver,
        }
    }

    async fn sign(
        &self,
        factor_sources: FactorSources,
        derivation_paths: Vec<DerivationPath>,
        payload: PayloadToSign,
    ) -> Result<IdentifiedVecOf<HierarchicalDeterministicSignature>> {
        let mut signatures_from_all_factors =
            IdentifiedVecOf::<HierarchicalDeterministicSignature>::new();
        for factor in factor_sources {
            let signatures = match factor {
                FactorSource::Device { value } => {
                    self.use_device_factor_source_driver
                        .sign_with_device(
                            value.clone(),
                            derivation_paths.clone(),
                            payload.clone(),
                        )
                        .await?
                }
                _ => todo!(),
            };
            signatures_from_all_factors.extend(signatures);
        }

        Ok(signatures_from_all_factors)
    }
}

impl SargonOS {
    pub async fn transaction_header(
        &self,
        notary_public_key: Ed25519PublicKey,
        notary_is_signatory: bool,
    ) -> Result<TransactionHeader> {
        let gateway = &self.clients.gateway_client;
        let network_id = gateway.network_id();
        let start_epoch_inclusive = gateway.current_epoch().await?;
        let end_epoch_exclusive = Epoch::from(start_epoch_inclusive.0 + 10u64);

        let header = TransactionHeader::new(
            network_id,
            start_epoch_inclusive,
            end_epoch_exclusive,
            Nonce::random(),
            notary_public_key,
            notary_is_signatory,
            0,
        );
        Ok(header)
    }

    pub async fn build_intent(
        &self,
        manifest: TransactionManifest,
        notary_public_key: Ed25519PublicKey,
        notary_is_signatory: bool,
        message: impl Into<Message>,
    ) -> Result<TransactionIntent> {
        let header = self
            .transaction_header(notary_public_key, notary_is_signatory)
            .await?;
        TransactionIntent::new(header, manifest, message.into())
    }

    pub async fn compile_sign_and_notarize_transaction(
        &self,
        manifest: TransactionManifest,
        message: impl Into<Message>,
    ) -> Result<CompiledNotarizedIntent> {
        let ephemeral_notary = Ed25519PrivateKey::generate();
        let intent = self
            .build_intent(
                manifest,
                ephemeral_notary.public_key(),
                true,
                message,
            )
            .await?;
        let intent_hash = intent.intent_hash();
        // let signing = &self.components.signing;
        /*
        let intent_signature = private_key.sign_intent_hash(&intent_hash);

        let signed_intent = SignedIntent::new(
            intent,
            IntentSignatures::new([intent_signature]),
        )
        .unwrap();

        let notary_signature = private_key.notarize_hash(&signed_intent.hash());

        let notarized_transaction =
            NotarizedTransaction::new(signed_intent, notary_signature).unwrap();

        let tx_id = timeout(
            MAX,
            gateway_client.submit_notarized_transaction(notarized_transaction),
        )
        .await
        .unwrap()
        .unwrap();

        Ok((address, tx_id))
        */
        todo!();
    }
}

#[uniffi::export]
impl SargonOS {
    pub async fn sign_submit_transaction(
        &self,
        _manifest: TransactionManifest,
    ) -> Result<IntentHash> {
        Err(CommonError::Unknown)
    }
}

#[uniffi::export]
impl SargonOS {
    pub async fn sign_submit_tx_use_faucet(
        &self,
        recipient: AccountAddress,
    ) -> Result<()> {
        // pub fn faucet(
        //     include_lock_fee_instruction: bool,
        //     address_of_receiving_account: &AccountAddress,
        // ) -> Self {
        let manifest = TransactionManifest::faucet(false, &recipient);

        // let intent = TransactionIntent
        // Err(CommonError::Unknown)
        todo!();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_rt::time::timeout;
    use std::time::Duration;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = SargonOS;

    #[actix_rt::test]
    async fn test_sign() {
        // ARRANGE (and ACT)
        let os = SUT::fast_boot().await;

        // let account = os
        //     .create_and_save_new_account(
        //         NetworkID::Stokenet,
        //         DisplayName::new("from").unwrap(),
        //     )
        //     .await
        //     .unwrap();
    }
}
