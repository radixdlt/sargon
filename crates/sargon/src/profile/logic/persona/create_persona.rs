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
        factor_instances_cache_client: &FactorInstancesCacheClient,
        key_derivation_interactors: Arc<dyn KeysDerivationInteractors>,
    ) -> Result<(FactorSourceID, Persona)> {
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
        factor_instances_cache_client: &FactorInstancesCacheClient,
        key_derivation_interactors: Arc<dyn KeysDerivationInteractors>,
    ) -> Result<(FactorSourceID, Persona)> {
        self.create_unsaved_persona_with_factor_source_with_derivation_outcome(
            factor_source,
            network_id,
            name,
            factor_instances_cache_client,
            key_derivation_interactors,
        )
        .await
        .map(|(x, y, _)| (x, y))
    }
    pub async fn create_unsaved_persona_with_factor_source_with_derivation_outcome(
        &self,
        factor_source: FactorSource,
        network_id: NetworkID,
        name: DisplayName,
        factor_instances_cache_client: &FactorInstancesCacheClient,
        key_derivation_interactors: Arc<dyn KeysDerivationInteractors>,
    ) -> Result<(
        FactorSourceID,
        Persona,
        FactorInstancesProviderOutcomeForFactor,
    )> {
        let (factor_source_id, personas, derivation_outcome) = self
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

        Ok((factor_source_id, persona, derivation_outcome))
    }

    /// Creates many new non securified personas **WITHOUT** adding them to Profile, using the *main* "Babylon"
    /// `DeviceFactorSource` and the "next" indices for this FactorSource as derivation paths.
    ///
    /// If you want to add the personas to Profile, call `add_personas(personas)`
    ///
    /// Returns a tuple `(FactorSourceID, Personas)` where FactorSourceID is the ID
    /// of the FactorSource used to create the personas.
    pub async fn create_unsaved_personas_with_bdfs(
        &self,
        network_id: NetworkID,
        count: u16,
        factor_instances_cache_client: &FactorInstancesCacheClient,
        key_derivation_interactors: Arc<dyn KeysDerivationInteractors>,
        get_name: impl Fn(u32) -> DisplayName, // name of persona at index
    ) -> Result<(FactorSourceID, Personas)> {
        let bdfs = self.bdfs();
        self.create_unsaved_personas_with_factor_source(
            bdfs.into(),
            network_id,
            count,
            factor_instances_cache_client,
            key_derivation_interactors,
            get_name,
        )
        .await
    }

    pub async fn create_unsaved_personas_with_factor_source(
        &self,
        factor_source: FactorSource,
        network_id: NetworkID,
        count: u16,
        factor_instances_cache_client: &FactorInstancesCacheClient,
        key_derivation_interactors: Arc<dyn KeysDerivationInteractors>,
        get_name: impl Fn(u32) -> DisplayName, // name of persona at index
    ) -> Result<(FactorSourceID, Personas)> {
        self.create_unsaved_personas_with_factor_source_with_derivation_outcome(
            factor_source,
            network_id,
            count,
            factor_instances_cache_client,
            key_derivation_interactors,
            get_name,
        )
        .await
        .map(|(x, y, _)| (x, y))
    }

    pub async fn create_unsaved_personas_with_factor_source_with_derivation_outcome(
        &self,
        factor_source: FactorSource,
        network_id: NetworkID,
        count: u16,
        factor_instances_cache_client: &FactorInstancesCacheClient,
        key_derivation_interactors: Arc<dyn KeysDerivationInteractors>,
        get_name: impl Fn(u32) -> DisplayName, // name of persona at index
    ) -> Result<(
        FactorSourceID,
        Personas,
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
