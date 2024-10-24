use crate::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct SignableWithEntities<S: Signable + Clone> {
    pub(crate) signable: S,
    id: S::SignableID,
    entities_requiring_auth: IndexSet<AccountOrPersona>,
}

impl <S: Signable + Clone> Identifiable for SignableWithEntities<S> {
    type ID = S::SignableID;

    fn id(&self) -> Self::ID {
        self.id.clone()
    }
}

impl <S: Signable + Clone> SignableWithEntities<S> {
    pub(crate) fn with(
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

    pub(crate) fn entities_requiring_auth(&self) -> IndexSet<AccountOrPersona> {
        self.entities_requiring_auth.clone().into_iter().collect()
    }

    pub(crate) fn extracting_from_profile(
        signable: &S,
        profile: &Profile,
    ) -> Result<Self> {
        let entities = signable.entities_requiring_signing(profile)?;

        Ok(Self::with(signable.clone(), entities))
    }
}

// -- Samples
impl SignableWithEntities<TransactionIntent> {
    #[allow(unused)]
    pub(crate) fn sample(
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

        let intent = TransactionIntent::sample_entity_addresses_requiring_auth(
            account_addresses,
            identity_addresses,
        );

        Self::with(intent, all_entities)
    }
}
