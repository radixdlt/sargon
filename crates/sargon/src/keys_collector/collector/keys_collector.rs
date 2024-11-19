use crate::prelude::*;

/// A coordinator which gathers public keys from several factor sources of different
/// kinds, in increasing friction order, for many transactions and for
/// potentially multiple entities and for many factor instances (derivation paths)
/// for each transaction.
///
/// By increasing friction order we mean, the quickest and easiest to use FactorSourceKind
/// is last; namely `DeviceFactorSource`, and the most tedious FactorSourceKind is
/// first; namely `LedgerFactorSource`, which user might also lack access to.
pub struct KeysCollector {
    /// Stateless immutable values used by the collector to gather public keys
    /// from factor sources.
    dependencies: KeysCollectorDependencies,

    /// Mutable internal state of the collector which builds up the list
    /// of public keys from each used factor source.
    state: RwLock<KeysCollectorState>,
}

impl KeysCollector {
    pub fn new(
        all_factor_sources_in_profile: impl IntoIterator<Item = FactorSource>,
        derivation_paths: impl Into<
            IndexMap<FactorSourceIDFromHash, IndexSet<DerivationPath>>,
        >,
        interactors: Arc<dyn KeysDerivationInteractors>,
    ) -> Result<Self> {
        let derivation_paths = derivation_paths.into();
        let preprocessor = KeysCollectorPreprocessor::new(derivation_paths);
        Self::with_preprocessor(
            all_factor_sources_in_profile
                .into_iter()
                .collect::<IndexSet<_>>(),
            interactors,
            preprocessor,
        )
    }

    fn with_preprocessor(
        all_factor_sources_in_profile: impl Into<IndexSet<FactorSource>>,
        interactors: Arc<dyn KeysDerivationInteractors>,
        preprocessor: KeysCollectorPreprocessor,
    ) -> Result<Self> {
        debug!("Init KeysCollector");
        let all_factor_sources_in_profile =
            all_factor_sources_in_profile.into();

        preprocessor
            .preprocess(all_factor_sources_in_profile)
            .map(|(s, f)| Self {
                dependencies: KeysCollectorDependencies::new(interactors, f),
                state: RwLock::new(s),
            })
    }
}

// === PUBLIC ===
impl KeysCollector {
    #[allow(unused)]
    pub async fn collect_keys(self) -> KeyDerivationOutcome {
        _ = self
            .derive_with_factors() // in decreasing "friction order"
            .await
            .inspect_err(|e| error!("Failed to use factor sources: {:#?}", e));
        self.state.into_inner().unwrap().outcome()
    }
}

// === PRIVATE ===
impl KeysCollector {
    async fn use_factor_sources(
        &self,
        factor_sources_of_kind: &FactorSourcesOfKind,
    ) -> Result<()> {
        let interactor = self
            .dependencies
            .interactors
            .interactor_for(factor_sources_of_kind.kind);
        let factor_sources = factor_sources_of_kind.factor_sources();
        match interactor {
            KeyDerivationInteractor::PolyFactor(interactor) => {
                // Prepare the request for the interactor
                trace!("Creating poly request for interactor");
                let request = self.request_for_parallel_interactor(
                    factor_sources
                        .into_iter()
                        .map(|f| f.id_from_hash())
                        .collect(),
                )?;
                trace!("Dispatching poly request to interactor: {:?}", request);
                let response = interactor.derive(request).await?;
                self.process_batch_response(response)?;
            }

            KeyDerivationInteractor::MonoFactor(interactor) => {
                for factor_source in factor_sources {
                    // Prepare the request for the interactor
                    trace!("Creating mono request for interactor");
                    let request = self.request_for_serial_interactor(
                        &factor_source.id_from_hash(),
                    )?;

                    trace!(
                        "Dispatching mono request to interactor: {:?}",
                        request
                    );
                    // Produce the results from the interactor
                    let response = interactor.derive(request).await?;

                    // Report the results back to the collector
                    self.process_batch_response(response)?;
                }
            }
        }
        Ok(())
    }

    /// In decreasing "friction order"
    async fn derive_with_factors(&self) -> Result<()> {
        for factor_sources_of_kind in self.dependencies.factors_of_kind.iter() {
            debug!(
                "Use(?) #{:?} factors of kind: {:?}",
                &factor_sources_of_kind.factor_sources().len(),
                &factor_sources_of_kind.kind
            );
            self.use_factor_sources(factor_sources_of_kind).await?;
        }
        Ok(())
    }

    fn input_for_interactor(
        &self,
        factor_source_id: &FactorSourceIDFromHash,
    ) -> Result<MonoFactorKeyDerivationRequest> {
        let keyring = self
            .state
            .try_read()
            .unwrap()
            .keyring_for(factor_source_id)?;
        assert_eq!(keyring.factors().len(), 0);
        let paths = keyring.paths.clone();
        Ok(MonoFactorKeyDerivationRequest::new(
            *factor_source_id,
            paths,
        ))
    }

    fn request_for_parallel_interactor(
        &self,
        factor_sources_ids: IndexSet<FactorSourceIDFromHash>,
    ) -> Result<PolyFactorKeyDerivationRequest> {
        let per_factor_source = factor_sources_ids
            .into_iter()
            .map(|f| self.input_for_interactor(&f))
            .collect::<Result<Vec<MonoFactorKeyDerivationRequest>>>()?;
        Ok(PolyFactorKeyDerivationRequest::new(
            per_factor_source
                .into_iter()
                .map(|r| (r.factor_source_id, r))
                .collect(),
        ))
    }

    fn request_for_serial_interactor(
        &self,
        factor_source_id: &FactorSourceIDFromHash,
    ) -> Result<MonoFactorKeyDerivationRequest> {
        self.input_for_interactor(factor_source_id)
    }

    fn process_batch_response(
        &self,
        response: KeyDerivationResponse,
    ) -> Result<()> {
        self.state
            .try_write()
            .unwrap()
            .process_batch_response(response)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[actix_rt::test]
    async fn valid() {
        let f0 = FactorSource::sample_ledger();
        let f1 = FactorSource::sample_device();
        let f2 = FactorSource::sample_device_babylon_other();
        let f3 = FactorSource::sample_arculus();

        let paths = IndexMap::<_, _>::from_iter([
            (
                f0.id_from_hash(),
                IndexSet::<_>::from_iter([
                    DerivationPath::for_entity(
                        NetworkID::Mainnet,
                        CAP26EntityKind::Account,
                        CAP26KeyKind::TransactionSigning,
                        Hardened::Securified(SecurifiedU30::ZERO),
                    ),
                    DerivationPath::for_entity(
                        NetworkID::Mainnet,
                        CAP26EntityKind::Account,
                        CAP26KeyKind::TransactionSigning,
                        Hardened::Securified(SecurifiedU30::ONE),
                    ),
                    DerivationPath::for_entity(
                        NetworkID::Stokenet,
                        CAP26EntityKind::Account,
                        CAP26KeyKind::TransactionSigning,
                        Hardened::Unsecurified(UnsecurifiedHardened::TWO),
                    ),
                ]),
            ),
            (
                f1.id_from_hash(),
                IndexSet::<_>::just(DerivationPath::for_entity(
                    NetworkID::Stokenet,
                    CAP26EntityKind::Account,
                    CAP26KeyKind::TransactionSigning,
                    Hardened::Unsecurified(UnsecurifiedHardened::THREE),
                )),
            ),
            (
                f2.id_from_hash(),
                IndexSet::<_>::just(DerivationPath::for_entity(
                    NetworkID::Mainnet,
                    CAP26EntityKind::Account,
                    CAP26KeyKind::TransactionSigning,
                    Hardened::Unsecurified(
                        UnsecurifiedHardened::try_from(4u32).unwrap(),
                    ),
                )),
            ),
            (
                f3.id_from_hash(),
                IndexSet::<_>::just(DerivationPath::for_entity(
                    NetworkID::Mainnet,
                    CAP26EntityKind::Identity,
                    CAP26KeyKind::AuthenticationSigning,
                    Hardened::Securified(
                        SecurifiedU30::try_from(5u32).unwrap(),
                    ),
                )),
            ),
        ]);

        let collector = KeysCollector::new(
            [f0, f1, f2, f3],
            paths.clone(),
            Arc::new(TestDerivationInteractors::default()),
        )
        .unwrap();

        let outcome = collector.collect_keys().await;
        let factors = outcome.all_factors().factor_instances();
        assert_eq!(
            factors.len(),
            paths
                .clone()
                .into_iter()
                .flat_map(|(_, v)| v)
                .collect::<IndexSet<_>>()
                .len(),
        );
    }
}
