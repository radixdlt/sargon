use crate::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SignableWithEntities<S: Signable> {
    pub signable: S,
    id: S::ID,
    entities_requiring_auth: IndexSet<AccountOrPersona>,
}

impl<S: Signable> Identifiable for SignableWithEntities<S> {
    type ID = S::ID;

    fn id(&self) -> Self::ID {
        self.id.clone()
    }
}

impl<S: Signable> SignableWithEntities<S> {
    pub fn with(
        signable: S,
        entities_requiring_auth: impl IntoIterator<
            Item = impl Into<AccountOrPersona>,
        >,
    ) -> Self {
        let id = signable.get_id();
        Self {
            signable,
            id,
            entities_requiring_auth: entities_requiring_auth
                .into_iter()
                .map(|i| i.into())
                .collect::<IndexSet<AccountOrPersona>>(),
        }
    }

    pub fn entities_requiring_auth(&self) -> IndexSet<AccountOrPersona> {
        self.entities_requiring_auth.clone().into_iter().collect()
    }

    pub fn extracting_from_profile(
        signable: &S,
        entity_querying: &impl GetEntityByAddress,
    ) -> Result<Self> {
        let entities = signable.entities_requiring_signing(entity_querying)?;

        Ok(Self::with(signable.clone(), entities))
    }
}

// -- Samples
impl<S: Signable + ProvidesSamplesByBuildingManifest> SignableWithEntities<S> {
    #[allow(unused)]
    pub fn sample(
        entities_requiring_auth: impl IntoIterator<
            Item = impl Into<AccountOrPersona>,
        >,
    ) -> Self {
        let mut account_addresses = Vec::new();
        let mut identity_addresses = Vec::new();

        let all_entities = entities_requiring_auth
            .into_iter()
            .map(|i| i.into())
            .collect::<Vec<_>>();

        all_entities.iter().for_each(|entity| match entity {
            AccountOrPersona::AccountEntity(account) => {
                account_addresses.push(account.address)
            }
            AccountOrPersona::PersonaEntity(persona) => {
                identity_addresses.push(persona.address)
            }
        });

        let intent = S::sample_entity_addresses_requiring_auth(
            account_addresses,
            identity_addresses,
        );

        Self::with(intent, all_entities)
    }
}
