use crate::prelude::*;

#[uniffi::export]
impl SargonOS {
    /// Create a new Persona and adds it to the active Profile.
    ///
    /// # Emits Event
    /// Emits `Event::ProfileModified { change: EventProfileModified::PersonaAdded }`
    pub async fn create_and_save_new_persona_with_factor_source(
        &self,
        factor_source: FactorSource,
        network_id: NetworkID,
        name: DisplayName,
        persona_data: Option<PersonaData>,
    ) -> Result<Persona> {
        self.wrapped
            .create_and_save_new_persona_with_factor_source(
                factor_source.into_internal(),
                network_id.into_internal(),
                name.into_internal(),
                persona_data.map(|v| v.into_internal()),
            )
            .await
            .into_result()
    }
}
