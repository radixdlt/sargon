use crate::prelude::*;

#[async_trait::async_trait]
pub trait OsDerivePublicKeys {
    async fn derive_public_keys(
        &self,
        derivation_paths: Vec<DerivationPath>,
        source: DerivePublicKeysSource,
    ) -> Result<Vec<HierarchicalDeterministicPublicKey>>;
}

#[derive(Clone, Debug, PartialEq)]
pub enum DerivePublicKeysSource {
    MnemonicWithPassphrase(MnemonicWithPassphrase),

    FactorSource(FactorSourceIDFromHash),
}

#[async_trait::async_trait]
impl OsDerivePublicKeys for SargonOS {
    async fn derive_public_keys(
        &self,
        derivation_paths: Vec<DerivationPath>,
        source: DerivePublicKeysSource,
    ) -> Result<Vec<HierarchicalDeterministicPublicKey>> {
        match source {
            DerivePublicKeysSource::MnemonicWithPassphrase(
                mnemonic_with_passphrase,
            ) => {
                let public_keys = mnemonic_with_passphrase
                    .derive_public_keys_vec(derivation_paths);
                Ok(public_keys)
            }
            DerivePublicKeysSource::FactorSource(
                factor_source_id_from_hash,
            ) => {
                let id = FactorSourceID::from(factor_source_id_from_hash);
                let factor_source =
                    self.factor_sources()?.get_id(id).cloned().ok_or(
                        CommonError::ProfileDoesNotContainFactorSourceWithID {
                            bad_value: id.to_string(),
                        },
                    )?;

                let collector = KeysCollector::new(
                    vec![factor_source],
                    IndexMap::just((
                        factor_source_id_from_hash,
                        IndexSet::from_iter(derivation_paths),
                    )),
                    self.keys_derivation_interactor(),
                    DerivationPurpose::AccountRecovery,
                )?;

                let pf_derived =
                    collector.collect_keys().await.factors_by_source;
                let result: Vec<HierarchicalDeterministicPublicKey> =
                    pf_derived
                        .get(&factor_source_id_from_hash)
                        .map(|set| {
                            set.iter().cloned().map(|a| a.public_key).collect()
                        })
                        .unwrap_or(Vec::new());
                Ok(result)
            }
        }
    }
}
