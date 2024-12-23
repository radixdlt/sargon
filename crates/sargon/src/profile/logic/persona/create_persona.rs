use crate::prelude::*;

impl Profile {
    pub async fn create_unsaved_persona_with_factor_source_with_derivation_outcome(
        &self,
        factor_source: FactorSource,
        network_id: NetworkID,
        name: DisplayName,
        factor_instances_cache_client: Arc<FactorInstancesCacheClient>,
        key_derivation_interactor: Arc<dyn KeyDerivationInteractor>,
    ) -> Result<(
        FactorSourceID,
        Persona,
        InstancesInCacheConsumer,
        FactorInstancesProviderOutcomeForFactor,
    )> {
        todo!()
        // let (
        //     factor_source_id,
        //     personas,
        //     instances_in_cache_consumer,
        //     derivation_outcome,
        // ) = self
        //     .create_unsaved_personas_with_factor_source_with_derivation_outcome(
        //         factor_source,
        //         network_id,
        //         1,
        //         factor_instances_cache_client,
        //         key_derivation_interactor,
        //         |_| name.clone(),
        //     )
        //     .await?;

        // let persona = personas
        //     .into_iter()
        //     .last()
        //     .expect("Should have created one persona");

        // Ok((
        //     factor_source_id,
        //     persona,
        //     instances_in_cache_consumer,
        //     derivation_outcome,
        // ))
    }

    pub async fn create_unsaved_personas_with_factor_source_with_derivation_outcome(
        &self,
        factor_source: FactorSource,
        network_id: NetworkID,
        count: u16,
        factor_instances_cache_client: Arc<FactorInstancesCacheClient>,
        key_derivation_interactor: Arc<dyn KeyDerivationInteractor>,
        get_name: impl Fn(u32) -> DisplayName, // name of persona at index
    ) -> Result<(
        FactorSourceID,
        Personas,
        InstancesInCacheConsumer,
        FactorInstancesProviderOutcomeForFactor,
    )> {
        todo!()
        // self.create_unsaved_entities_with_factor_source_with_derivation_outcome::<Persona>(
        //     factor_source,
        //     network_id,
        //     count,
        //     factor_instances_cache_client,
        //     key_derivation_interactor,
        //     get_name,
        // )
        // .await
    }
}
