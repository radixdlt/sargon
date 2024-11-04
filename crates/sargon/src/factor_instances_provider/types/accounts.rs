use crate::prelude::*;

pub type Accounts = Entities<Account>;
pub type Personas = Entities<Persona>;

/// A NonEmpty collection of Entities all on the SAME Network
/// but mixed if they are securified or unsecurified.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Entities<
    E: IsNetworkAware
        + HasEntityKindObjectSafe
        + Clone
        + std::hash::Hash
        + std::cmp::Eq,
> {
    pub network_id: NetworkID,
    entities: IndexSet<E>,
}

impl<
        E: IsNetworkAware
            + HasEntityKindObjectSafe
            + Clone
            + std::hash::Hash
            + std::cmp::Eq,
    > Entities<E>
{
    pub fn new(network_id: NetworkID, entities: IndexSet<E>) -> Result<Self> {
        if entities.is_empty() {
            return Err(CommonError::ExpectedNonEmptyCollection);
        }
        if let Some(e) = entities.iter().find(|e| e.network_id() != network_id)
        {
            return Err(CommonError::EntityOnWrongNetwork {
                entity_kind: e.get_entity_kind().to_string(),
                wrong_network: e.network_id(),
                expected_network: network_id,
            });
        }
        Ok(Self {
            network_id,
            entities,
        })
    }

    pub fn len(&self) -> usize {
        self.entities.len()
    }

    /// Should never be true, since we do not allow empty.
    pub fn is_empty(&self) -> bool {
        self.entities.is_empty()
    }

    pub fn network_id(&self) -> NetworkID {
        self.network_id
    }
}

impl<
        E: IsNetworkAware
            + HasEntityKindObjectSafe
            + Clone
            + std::hash::Hash
            + std::cmp::Eq,
    > IntoIterator for Entities<E>
{
    type Item = E;
    type IntoIter = <IndexSet<E> as IntoIterator>::IntoIter;
    fn into_iter(self) -> Self::IntoIter {
        self.entities.clone().into_iter()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    type Sut = Accounts;
    type Item = Account;
    #[test]
    fn empty_throws() {
        assert!(matches!(
            Sut::new(NetworkID::Mainnet, IndexSet::new()),
            Err(CommonError::ExpectedNonEmptyCollection)
        ));
    }
    #[test]
    fn wrong_network_single() {
        assert!(matches!(
            Sut::new(NetworkID::Stokenet, IndexSet::just(Item::sample())),
            Err(CommonError::EntityOnWrongNetwork { .. })
        ));
    }
    #[test]
    fn wrong_network_two() {
        assert!(matches!(
            Sut::new(
                NetworkID::Stokenet,
                IndexSet::from_iter([Item::sample_other(), Item::sample(),])
            ),
            Err(CommonError::EntityOnWrongNetwork { .. })
        ));
    }
    #[test]
    fn ok_new() {
        let sut = Sut::new(NetworkID::Mainnet, IndexSet::just(Item::sample()))
            .unwrap();
        assert!(!sut.is_empty());
    }
}
