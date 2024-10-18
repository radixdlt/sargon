use crate::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct TXToSign {
    pub(crate) intent: TransactionIntent,
    intent_hash: IntentHash,
    entities_requiring_auth: IndexSet<AccountOrPersona>,
}

impl Identifiable for TXToSign {
    type ID = IntentHash;

    fn id(&self) -> Self::ID {
        self.intent_hash.clone()
    }
}

impl TXToSign {
    pub(crate) fn with(
        intent: TransactionIntent,
        entities_requiring_auth: impl IntoIterator<
            Item = impl Into<AccountOrPersona>,
        >,
    ) -> Self {
        let intent_hash = intent.intent_hash().clone();
        Self {
            intent,
            intent_hash,
            entities_requiring_auth: entities_requiring_auth
                .into_iter()
                .map(|i| i.into())
                .collect::<IndexSet<AccountOrPersona>>(),
        }
    }

    pub(crate) fn entities_requiring_auth(&self) -> IndexSet<AccountOrPersona> {
        self.entities_requiring_auth.clone().into_iter().collect()
    }

    pub(crate) fn extracting_from_intent_and_profile(
        intent: &TransactionIntent,
        profile: &Profile,
    ) -> Result<Self> {
        let entities_requiring_auth =
            ExtractorOfEntitiesRequiringAuth::extract(
                profile,
                intent.manifest_summary().clone(),
            )?;

        Ok(Self::with(intent.clone(), entities_requiring_auth))
    }
}

// -- Samples
impl TXToSign {
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
