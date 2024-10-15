use crate::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq, std::hash::Hash)]
pub(crate) struct TXToSign {
    pub(crate) intent_hash: IntentHash,
    entities_requiring_auth: Vec<AccountOrPersona>, // should be a set but Sets are not `Hash`.
}

impl TXToSign {
    pub(crate) fn with(
        intent_hash: IntentHash,
        entities_requiring_auth: impl IntoIterator<
            Item = impl Into<AccountOrPersona>,
        >,
    ) -> Self {
        Self {
            intent_hash,
            entities_requiring_auth: entities_requiring_auth
                .into_iter()
                .map(|i| i.into())
                .collect_vec(),
        }
    }

    pub(crate) fn entities_requiring_auth(&self) -> IndexSet<AccountOrPersona> {
        self.entities_requiring_auth.clone().into_iter().collect()
    }

    pub(crate) fn extracting_from_intent_and_profile(
        intent: &TransactionIntent,
        profile: &Profile,
    ) -> Result<Self> {
        let intent_hash = intent.intent_hash().clone();
        let summary = intent.manifest_summary();
        let mut entities_requiring_auth: IndexSet<AccountOrPersona> =
            IndexSet::new();

        let accounts = summary
            .addresses_of_accounts_requiring_auth
            .iter()
            .map(|a| profile.account_by_address(*a))
            .collect::<Result<Vec<_>>>()?;

        entities_requiring_auth.extend(
            accounts
                .into_iter()
                .map(AccountOrPersona::from)
                .collect_vec(),
        );

        let personas = summary
            .addresses_of_personas_requiring_auth
            .into_iter()
            .map(|a| profile.persona_by_address(a))
            .collect::<Result<Vec<_>>>()?;

        entities_requiring_auth.extend(
            personas
                .into_iter()
                .map(AccountOrPersona::from)
                .collect_vec(),
        );

        Ok(Self::with(intent_hash, entities_requiring_auth))
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
        Self::with(
            IntentHash::new(
                Hash::from(Exactly32Bytes::generate()),
                NetworkID::Mainnet,
            ),
            entities_requiring_auth,
        )
    }
}
