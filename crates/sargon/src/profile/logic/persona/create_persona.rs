use crate::prelude::*;

impl Profile {
    /// Creates a new non securified persona **WITHOUT** adding it to Profile, using the *main* "Babylon"
    /// `DeviceFactorSource` and the "next" index for this FactorSource as derivation path.
    ///
    /// If you want to add it to Profile, call `add_persona(persona)`
    ///
    /// Returns a tuple `(FactorSourceID, Persona)` where FactorSourceID is the ID
    /// of the FactorSource used to create the persona.
    pub async fn create_unsaved_persona_with_bdfs(
        &self,
        network_id: NetworkID,
        name: DisplayName,
        factor_instances_cache_client: Arc<FactorInstancesCacheClient>,
        key_derivation_interactors: Arc<dyn KeysDerivationInteractors>,
    ) -> Result<(FactorSourceID, Persona, InstancesInCacheConsumer)> {
        let bdfs = self.bdfs();
        self.create_unsaved_persona_with_factor_source(
            bdfs.into(),
            network_id,
            name,
            factor_instances_cache_client,
            key_derivation_interactors,
        )
        .await
    }

    /// Creates a new non securified persona **WITHOUT** adding it to Profile, using the *main* "Babylon"
    /// `DeviceFactorSource` and the "next" index for this FactorSource as derivation path.
    ///
    /// If you want to add it to Profile, call `add_persona(persona)`
    ///
    /// Returns a tuple `(FactorSourceID, Persona)` where FactorSourceID is the ID
    /// of the FactorSource used to create the persona.
    pub async fn create_unsaved_persona_with_factor_source(
        &self,
        factor_source: FactorSource,
        network_id: NetworkID,
        name: DisplayName,
        factor_instances_cache_client: Arc<FactorInstancesCacheClient>,
        key_derivation_interactors: Arc<dyn KeysDerivationInteractors>,
    ) -> Result<(FactorSourceID, Persona, InstancesInCacheConsumer)> {
        self.create_unsaved_persona_with_factor_source_with_derivation_outcome(
            factor_source,
            network_id,
            name,
            factor_instances_cache_client,
            key_derivation_interactors,
        )
        .await
        .map(|(x, y, z, _)| (x, y, z))
    }
    pub async fn create_unsaved_persona_with_factor_source_with_derivation_outcome(
        &self,
        factor_source: FactorSource,
        network_id: NetworkID,
        name: DisplayName,
        factor_instances_cache_client: Arc<FactorInstancesCacheClient>,
        key_derivation_interactors: Arc<dyn KeysDerivationInteractors>,
    ) -> Result<(
        FactorSourceID,
        Persona,
        InstancesInCacheConsumer,
        FactorInstancesProviderOutcomeForFactor,
    )> {
        let (
            factor_source_id,
            personas,
            instances_in_cache_consumer,
            derivation_outcome,
        ) = self
            .create_unsaved_personas_with_factor_source_with_derivation_outcome(
                factor_source,
                network_id,
                1,
                factor_instances_cache_client,
                key_derivation_interactors,
                |_| name.clone(),
            )
            .await?;

        let persona = personas
            .into_iter()
            .last()
            .expect("Should have created one persona");

        Ok((
            factor_source_id,
            persona,
            instances_in_cache_consumer,
            derivation_outcome,
        ))
    }

    pub async fn create_unsaved_personas_with_factor_source_with_derivation_outcome(
        &self,
        factor_source: FactorSource,
        network_id: NetworkID,
        count: u16,
        factor_instances_cache_client: Arc<FactorInstancesCacheClient>,
        key_derivation_interactors: Arc<dyn KeysDerivationInteractors>,
        get_name: impl Fn(u32) -> DisplayName, // name of persona at index
    ) -> Result<(
        FactorSourceID,
        Personas,
        InstancesInCacheConsumer,
        FactorInstancesProviderOutcomeForFactor,
    )> {
        self.create_unsaved_entities_with_factor_source_with_derivation_outcome::<Persona>(
            factor_source,
            network_id,
            count,
            factor_instances_cache_client,
            key_derivation_interactors,
            get_name,
        )
        .await
    }
}
