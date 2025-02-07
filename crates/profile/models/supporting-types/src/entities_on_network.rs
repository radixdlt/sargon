use crate::prelude::*;

#[derive(Debug, Clone, PartialEq)]
pub struct EntitiesOnNetwork {
    pub network_id: NetworkID,

    /// might be empty
    pub unsecurified_accounts_on_network: IdentifiedVecOf<UnsecurifiedAccount>,

    /// might be empty
    pub securified_accounts_on_network: IdentifiedVecOf<SecurifiedAccount>,

    /// might be empty
    pub unsecurified_personas_on_network: IdentifiedVecOf<UnsecurifiedPersona>,

    /// might be empty
    pub securified_personas_on_network: IdentifiedVecOf<SecurifiedPersona>,
}

impl EntitiesOnNetwork {
    /// # Throws
    /// Throws if any entity in `splitting` is not on the network `network_id`
    pub fn with_split(
        network_id: NetworkID,
        unsecurified_accounts_on_network: IdentifiedVecOf<UnsecurifiedAccount>,
        securified_accounts_on_network: IdentifiedVecOf<SecurifiedAccount>,
        unsecurified_personas_on_network: IdentifiedVecOf<UnsecurifiedPersona>,
        securified_personas_on_network: IdentifiedVecOf<SecurifiedPersona>,
    ) -> Result<Self> {
        if let Some(entity_wrong_network) = unsecurified_accounts_on_network
            .iter()
            .find(|e| e.network_id() != network_id)
            .map(|e| e.network_id())
            .or(securified_accounts_on_network
                .iter()
                .find(|e| e.network_id() != network_id)
                .map(|e| e.network_id()))
            .or(unsecurified_personas_on_network
                .iter()
                .find(|e| e.network_id() != network_id)
                .map(|e| e.network_id()))
            .or(securified_personas_on_network
                .iter()
                .find(|e| e.network_id() != network_id)
                .map(|e| e.network_id()))
        {
            return Err(CommonError::NetworkDiscrepancy {
                expected: network_id.to_string(),
                actual: entity_wrong_network.to_string(),
            });
        }
        Ok(Self {
            network_id,
            unsecurified_accounts_on_network,
            securified_accounts_on_network,
            unsecurified_personas_on_network,
            securified_personas_on_network,
        })
    }

    /// # Throws
    /// Throws if any entity in `splitting` is not on the network `network_id`
    pub fn new(
        network_id: NetworkID,
        splitting: impl IntoIterator<Item = AccountOrPersona>,
    ) -> Result<Self> {
        let splitting = splitting.into_iter().collect::<IdentifiedVecOf<_>>();
        let len = splitting.len();
        let unsecurified = splitting
            .iter()
            .filter(|e| !e.is_securified())
            .map(|e| {
                AnyUnsecurifiedEntity::new(e).expect("Filtered before mapped")
            })
            .collect::<IdentifiedVecOf<_>>();

        let securified = splitting
            .iter()
            .filter(|e| e.is_securified())
            .map(|e| {
                AnySecurifiedEntity::new(e).expect("Filtered before mapped")
            })
            .collect::<IdentifiedVecOf<_>>();

        let _self = Self::with_split(
            network_id,
            unsecurified
                .iter()
                .filter_map(|e| UnsecurifiedAccount::try_from(e).ok())
                .collect(),
            securified
                .iter()
                .filter_map(|e| SecurifiedAccount::try_from(e).ok())
                .collect(),
            unsecurified
                .iter()
                .filter_map(|e| UnsecurifiedPersona::try_from(e).ok())
                .collect(),
            securified
                .iter()
                .filter_map(|e| SecurifiedPersona::try_from(e).ok())
                .collect(),
        )?;

        assert_eq!(
            len,
            _self.securified_accounts_on_network.len()
                + _self.unsecurified_accounts_on_network.len()
                + _self.securified_personas_on_network.len()
                + _self.unsecurified_personas_on_network.len(),
            "Internal error, incorrect implementation of SecurifiedEntity or UnsecurifiedEntity"
        );

        Ok(_self)
    }

    pub fn unsecurified_erased(
        &self,
    ) -> IdentifiedVecOf<AnyUnsecurifiedEntity> {
        let mut entities = IdentifiedVecOf::new();
        entities.extend(
            self.unsecurified_accounts_on_network
                .iter()
                .map(|e| e.into()),
        );

        entities.extend(
            self.unsecurified_personas_on_network
                .iter()
                .map(|e| e.into()),
        );

        entities
    }

    pub fn securified_erased(&self) -> IdentifiedVecOf<AnySecurifiedEntity> {
        let mut entities = IdentifiedVecOf::new();
        entities.extend(
            self.securified_accounts_on_network.iter().map(|e| e.into()),
        );

        entities.extend(
            self.securified_personas_on_network.iter().map(|e| e.into()),
        );

        entities
    }

    fn to_entities(&self) -> IdentifiedVecOf<AccountOrPersona> {
        let mut entities = IdentifiedVecOf::new();
        entities.extend(self.unsecurified_erased().iter().map(|e| e.entity));
        entities.extend(self.securified_erased().iter().map(|e| e.entity));

        entities
    }

    pub fn all(&self, predicate: impl Fn(AccountOrPersona) -> bool) -> bool {
        self.to_entities().iter().all(predicate)
    }
}
