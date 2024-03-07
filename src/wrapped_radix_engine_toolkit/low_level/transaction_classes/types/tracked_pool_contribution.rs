use crate::prelude::*;

use radix_engine_toolkit::transaction_types::TrackedPoolContribution as RetTrackedPoolContribution;

#[derive(Clone, Debug, PartialEq, Eq, uniffi::Record)]
pub struct TrackedPoolContribution {
    pub pool_address: PoolAddress,

    /* Input */
    pub contributed_resources: HashMap<ResourceAddress, Decimal>,

    /* Output */
    pub pool_units_resource_address: ResourceAddress,
    pub pool_units_amount: Decimal192,
}

impl From<(RetTrackedPoolContribution, NetworkID)> for TrackedPoolContribution {
    fn from(value: (RetTrackedPoolContribution, NetworkID)) -> Self {
        let (ret, network_id) = value;
        Self {
            pool_address: (ret.pool_address, network_id).into(),
            contributed_resources: to_hashmap_network_aware_key(
                ret.contributed_resources,
                network_id,
            ),
            pool_units_resource_address: (
                ret.pool_units_resource_address,
                network_id,
            )
                .into(),
            pool_units_amount: ret.pool_units_amount.into(),
        }
    }
}

pub(crate) fn to_hashmap_network_aware_kv<K, V, L, U>(
    values: impl IntoIterator<Item = (K, V)>,
    network_id: NetworkID,
) -> HashMap<L, U>
where
    L: Eq + std::hash::Hash + From<(K, NetworkID)>,
    U: From<(V, NetworkID)>,
{
    values
        .into_iter()
        .map(|(k, v)| (L::from((k, network_id)), U::from((v, network_id))))
        .collect::<HashMap<L, U>>()
}

pub(crate) fn to_hashmap_network_aware_value<K, V, L, U>(
    values: impl IntoIterator<Item = (K, V)>,
    network_id: NetworkID,
) -> HashMap<L, U>
where
    L: Eq + std::hash::Hash + From<K>,
    U: From<(V, NetworkID)>,
{
    values
        .into_iter()
        .map(|(k, v)| (L::from(k), U::from((v, network_id))))
        .collect::<HashMap<L, U>>()
}

pub(crate) fn to_hashmap_network_aware_key<K, V, L, U>(
    values: impl IntoIterator<Item = (K, V)>,
    network_id: NetworkID,
) -> HashMap<L, U>
where
    L: Eq + std::hash::Hash + From<(K, NetworkID)>,
    U: From<V>,
{
    values
        .into_iter()
        .map(|(k, v)| (L::from((k, network_id)), U::from(v)))
        .collect::<HashMap<L, U>>()
}
