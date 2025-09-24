use crate::prelude::*;

/// A coordinator which gathers public keys from several factor sources of different
/// kinds, in decreasing friction order, for many transactions and for
/// potentially multiple entities and for many factor instances (derivation paths)
/// for each transaction.
///
/// By decreasing friction order we mean, the quickest and easiest to use FactorSourceKind
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
        interactor: Arc<dyn KeyDerivationInteractor>,
        derivation_purpose: DerivationPurpose,
    ) -> Result<Self> {
        let derivation_paths = derivation_paths.into();
        let preprocessor = KeysCollectorPreprocessor::new(derivation_paths);
        Self::with_preprocessor(
            all_factor_sources_in_profile
                .into_iter()
                .collect::<IndexSet<_>>(),
            interactor,
            preprocessor,
            derivation_purpose,
        )
    }

    fn with_preprocessor(
        all_factor_sources_in_profile: impl Into<IndexSet<FactorSource>>,
        interactor: Arc<dyn KeyDerivationInteractor>,
        preprocessor: KeysCollectorPreprocessor,
        derivation_purpose: DerivationPurpose,
    ) -> Result<Self> {
        debug!("Init KeysCollector");
        let all_factor_sources_in_profile =
            all_factor_sources_in_profile.into();

        preprocessor
            .preprocess(all_factor_sources_in_profile)
            .map(|(s, f)| Self {
                dependencies: KeysCollectorDependencies::new(
                    interactor,
                    f,
                    derivation_purpose,
                ),
                state: RwLock::new(s),
            })
    }
}

// === PUBLIC ===
impl KeysCollector {
    #[allow(unused)]
    pub async fn collect_keys(self) -> Result<KeyDerivationOutcome> {
        _ = self
            .derive_with_factors() // in decreasing "friction order"
            .await?;
        Ok(self.state.into_inner().unwrap().outcome())
    }
}

// === PRIVATE ===
impl KeysCollector {
    async fn use_factor_sources(
        &self,
        factor_sources_of_kind: &FactorSourcesOfKind,
    ) -> Result<()> {
        let interactor = self.dependencies.interactor.clone();
        let factor_sources = factor_sources_of_kind.factor_sources();

        if factor_sources_of_kind.kind == FactorSourceKind::Device {
            for factor_source in factor_sources {
                // Prepare the request for the interactor
                trace!("Creating mono request for interactor");
                let request = self.request_for_serial_interactor(
                    &factor_source.id_from_hash(),
                )?;

                trace!("Dispatching mono request to interactor: {:?}", request);
                // Produce the results from the interactor
                let response = interactor.derive(request).await?;

                // Report the results back to the collector
                self.process_batch_response(response)?;
            }
        } else {
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
    ) -> Result<(FactorSourceIDFromHash, IndexSet<DerivationPath>)> {
        let keyring = self
            .state
            .try_read()
            .unwrap()
            .keyring_for(factor_source_id)?;
        assert_eq!(keyring.factors().len(), 0);
        let paths = keyring.paths.clone();
        Ok((*factor_source_id, paths))
    }

    fn request_for_parallel_interactor(
        &self,
        factor_sources_ids: IndexSet<FactorSourceIDFromHash>,
    ) -> Result<KeyDerivationRequest> {
        let per_factor_source =
            factor_sources_ids
                .into_iter()
                .map(|f| self.input_for_interactor(&f))
                .collect::<Result<
                    Vec<(FactorSourceIDFromHash, IndexSet<DerivationPath>)>,
                >>()?;
        Ok(KeyDerivationRequest::new(
            self.dependencies.derivation_purpose.clone(),
            per_factor_source.into_iter().collect(),
        ))
    }

    fn request_for_serial_interactor(
        &self,
        factor_source_id: &FactorSourceIDFromHash,
    ) -> Result<KeyDerivationRequest> {
        let (id, derivation_paths) =
            self.input_for_interactor(factor_source_id)?;

        Ok(KeyDerivationRequest::new_mono_factor(
            self.dependencies.derivation_purpose.clone(),
            id,
            derivation_paths,
        ))
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
