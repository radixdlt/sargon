use crate::prelude::*;

/// The badges to look for in the account to know whether it can satisfy some withdraw rules:
/// only those the rules ask for, and only as much of them as the rules ask for.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct BadgesToLookUp {
    /// The fungible badges: their amount in the account is needed.
    pub fungibles: IndexSet<ResourceAddress>,

    /// The non-fungible badges any id of which will do, with how many ids are needed at most.
    pub non_fungibles: IndexMap<ResourceAddress, u64>,

    /// The specific non-fungible badges asked for, by resource.
    pub non_fungible_ids:
        IndexMap<ResourceAddress, IndexSet<NonFungibleLocalId>>,
}

impl BadgesToLookUp {
    pub fn is_empty(&self) -> bool {
        self.fungibles.is_empty()
            && self.non_fungibles.is_empty()
            && self.non_fungible_ids.is_empty()
    }

    pub(crate) fn insert_resource(
        &mut self,
        resource: ResourceAddress,
        ids_needed: u64,
    ) {
        if resource.is_fungible() {
            self.fungibles.insert(resource);
        } else {
            let needed = self.non_fungibles.entry(resource).or_insert(0);
            *needed = (*needed).max(ids_needed);
        }
    }

    pub(crate) fn insert_non_fungible(&mut self, id: NonFungibleGlobalId) {
        self.non_fungible_ids
            .entry(id.resource_address)
            .or_default()
            .insert(id.non_fungible_local_id);
    }
}

/// What an account holds of the badges looked up (see `BadgesToLookUp`): the amount of each
/// fungible badge, and ids of each non-fungible badge (not necessarily all of them, but at least
/// as many as asked for, and the specific ones asked for that the account holds).
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct BadgeHoldings {
    pub fungibles: IndexMap<ResourceAddress, Decimal192>,
    pub non_fungibles: IndexMap<ResourceAddress, IndexSet<NonFungibleLocalId>>,
}

impl BadgeHoldings {
    pub fn new(
        fungibles: impl IntoIterator<Item = (ResourceAddress, Decimal192)>,
        non_fungibles: impl IntoIterator<
            Item = (ResourceAddress, IndexSet<NonFungibleLocalId>),
        >,
    ) -> Self {
        let mut holdings = Self::default();
        for (resource, amount) in fungibles {
            holdings.insert_fungible(resource, amount);
        }
        for (resource, ids) in non_fungibles {
            holdings.insert_non_fungibles(resource, ids);
        }
        holdings
    }

    pub fn insert_fungible(
        &mut self,
        resource: ResourceAddress,
        amount: Decimal192,
    ) {
        let held = self.fungibles.entry(resource).or_insert(Decimal192::zero());
        *held = *held + amount;
    }

    pub fn insert_non_fungibles(
        &mut self,
        resource: ResourceAddress,
        ids: impl IntoIterator<Item = NonFungibleLocalId>,
    ) {
        self.non_fungibles.entry(resource).or_default().extend(ids);
    }

    pub(crate) fn amount_of(&self, resource: &ResourceAddress) -> Decimal192 {
        self.fungibles
            .get(resource)
            .cloned()
            .unwrap_or(Decimal192::zero())
    }

    pub(crate) fn ids_of(
        &self,
        resource: &ResourceAddress,
    ) -> IndexSet<NonFungibleLocalId> {
        self.non_fungibles
            .get(resource)
            .cloned()
            .unwrap_or_default()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn look_up_keeps_the_most_ids_needed_per_resource() {
        let nft = ResourceAddress::sample_mainnet_nft_other();
        let mut sut = BadgesToLookUp::default();
        assert!(sut.is_empty());
        sut.insert_resource(nft, 1);
        sut.insert_resource(nft, 3);
        sut.insert_resource(nft, 2);
        sut.insert_resource(ResourceAddress::sample_mainnet_candy(), 1);
        assert_eq!(sut.non_fungibles.get(&nft), Some(&3));
        assert_eq!(
            sut.fungibles,
            IndexSet::just(ResourceAddress::sample_mainnet_candy())
        );
        assert!(!sut.is_empty());
    }

    #[test]
    fn holdings_add_up() {
        let candy = ResourceAddress::sample_mainnet_candy();
        let nft = ResourceAddress::sample_mainnet_nft_other();
        let mut sut = BadgeHoldings::new(
            [(candy, Decimal192::one())],
            [(nft, IndexSet::just(NonFungibleLocalId::integer(1)))],
        );
        sut.insert_fungible(candy, Decimal192::two());
        sut.insert_non_fungibles(nft, [NonFungibleLocalId::integer(2)]);
        assert_eq!(sut.amount_of(&candy), Decimal192::three());
        assert_eq!(sut.ids_of(&nft).len(), 2);
        assert_eq!(
            sut.amount_of(&ResourceAddress::sample_mainnet_xrd()),
            Decimal192::zero()
        );
    }
}
