use crate::prelude::*;
use sargon::OsArculusCard;
use sargon::ArculusCardState as InternalArculusCardState;

#[derive(Clone, PartialEq, Eq, InternalConversion, uniffi::Enum)]
pub enum ArculusCardState {
    NotConfigured,
    Configured(FactorSourceIDFromHash),
}

#[uniffi::export]
impl SargonOS {
    async fn arculus_get_card_state(&self) -> Result<ArculusCardState> {
        self.wrapped
            .arculus_get_card_state()
            .await
            .into_result()
    }

    pub async fn arculus_configure_card(
        &self,
    ) -> Result<FactorSourceIDFromHash> {
        self.wrapped
            .arculus_configure_card()
            .await
            .into_result()
    }

    pub async fn arculus_configure_card_with_mnemonic(
        &self,
        mnemonic: Mnemonic,
    ) -> Result<FactorSourceIDFromHash> {
        self.wrapped
            .arculus_configure_card_with_mnemonic(mnemonic.into_internal())
            .await
            .into_result()
    }

    pub async fn arculus_card_derive_public_keys(
        &self,
        factor_source: ArculusCardFactorSource,
        paths: Vec<DerivationPath>,
    ) -> Result<Vec<HierarchicalDeterministicPublicKey>> {
        self.wrapped
            .arculus_card_derive_public_keys(
                factor_source.into_internal(),
                paths
                    .into_iter()
                    .map(|path| path.into_internal())
                    .collect::<sargon::IndexSet<_>>(),
            )
            .await
            .map(|keys| {
                keys.into_iter()
                    .map(|key| key)
                    .collect::<Vec<sargon::HierarchicalDeterministicPublicKey>>(
                    )
            })
            .into_iter_result()
    }

    pub async fn arculus_card_sign_hashes(
        &self,
        factor_source: ArculusCardFactorSource,
        hashes: HashMap<Hash, Vec<DerivationPath>>,
    ) -> Result<HashMap<Hash, Vec<SignatureWithPublicKey>>> {
        self.wrapped
            .arculus_card_sign_hashes(
                factor_source.into_internal(),
                hashes.into_iter().map(|(hash, paths)| {
                    (hash.into_internal(), paths.into_iter().map(|path| path.into_internal()).collect::<sargon::IndexSet<_>>())
                }).collect::<sargon::IndexMap<sargon::Hash, IndexSet<sargon::DerivationPath>>>(),
            )
            .await
            .map(|signatures| {
                signatures.into_iter()
                    .map(|(hash, sigs)| {
                        (hash.into(), sigs.into_iter().map(|sig| sig.into()).collect::<Vec<SignatureWithPublicKey>>())
                    })
                    .collect::<HashMap<Hash, Vec<SignatureWithPublicKey>>>()
            })
            .into_result()
    }

    pub async fn arculus_card_sign_hash(
        &self,
        factor_source: ArculusCardFactorSource,
        hash: Hash,
        paths: Vec<DerivationPath>,
    ) -> Result<Vec<SignatureWithPublicKey>> {
        self.wrapped.arculus_card_sign_hash(
            factor_source.into_internal(),
            hash.into(),
            paths.into_iter().map(|path| path.into_internal()).collect::<sargon::IndexSet<_>>()
        )
        .await
        .into_iter_result()
    }

    pub async fn arculus_card_reset(&self) -> Result<()> {
        self.wrapped.arculus_card_reset().await.into_result()
    }
}
