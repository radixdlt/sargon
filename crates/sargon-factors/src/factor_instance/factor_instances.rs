use crate::prelude::*;

/// A collection of factor instances.
#[derive(
    Default, Clone, PartialEq, Eq, Serialize, Deserialize, derive_more::Debug,
)]
#[debug("FIS[{:?}]", self.factor_instances)]
#[serde(transparent)]
pub struct FactorInstances {
    #[allow(dead_code)]
    #[doc(hidden)]
    #[serde(skip)]
    #[debug(skip)]
    __hidden: HiddenConstructor,
    factor_instances: IndexSet<HierarchicalDeterministicFactorInstance>,
}

impl AppendableCollection for FactorInstances {
    type Element = HierarchicalDeterministicFactorInstance;

    fn append<T: IntoIterator<Item = Self::Element>>(&mut self, iter: T) {
        self.extend(iter)
    }
}


impl FactorInstances {
    pub fn extend(
        &mut self,
        instances: impl IntoIterator<Item = HierarchicalDeterministicFactorInstance>,
    ) {
        self.factor_instances.extend(instances);
    }

    pub fn shift_remove_index(
        &mut self,
        index: usize,
    ) -> HierarchicalDeterministicFactorInstance {
        self.factor_instances
            .shift_remove_index(index)
            .expect("correct index")
    }

    pub fn shift_remove(
        &mut self,
        item: &HierarchicalDeterministicFactorInstance,
    ) -> HierarchicalDeterministicFactorInstance {
        let idx = self
            .factor_instances
            .get_index_of(item)
            .expect("existing item");
        self.shift_remove_index(idx)
    }

    fn first_of_key_kind(
        &self,
        key_kind: CAP26KeyKind,
    ) -> Option<HierarchicalDeterministicFactorInstance> {
        self.factor_instances
            .iter()
            .find(|i| i.get_key_kind() == key_kind)
            .cloned()
    }

    pub fn first_transaction_signing(
        &self,
    ) -> Option<HierarchicalDeterministicFactorInstance> {
        self.first_of_key_kind(CAP26KeyKind::TransactionSigning)
    }

    pub fn first_authentication_signing(
        &self,
    ) -> Option<HierarchicalDeterministicFactorInstance> {
        self.first_of_key_kind(CAP26KeyKind::AuthenticationSigning)
    }

    pub fn split_at(self, mid: usize) -> (Self, Self) {
        let instances = self.factor_instances.into_iter().collect_vec();
        let (head, tail) = instances.split_at(mid);
        (Self::from(head), Self::from(tail))
    }
}

impl From<&[HierarchicalDeterministicFactorInstance]> for FactorInstances {
    fn from(value: &[HierarchicalDeterministicFactorInstance]) -> Self {
        Self::from(
            IndexSet::<HierarchicalDeterministicFactorInstance>::from_iter(
                value.iter().cloned(),
            ),
        )
    }
}

impl From<IndexSet<HierarchicalDeterministicFactorInstance>>
    for FactorInstances
{
    fn from(
        instances: IndexSet<HierarchicalDeterministicFactorInstance>,
    ) -> Self {
        Self::new(instances)
    }
}

impl FactorInstances {
    pub fn is_empty(&self) -> bool {
        self.factor_instances.is_empty()
    }

    pub fn len(&self) -> usize {
        self.factor_instances.len()
    }
}

impl IntoIterator for FactorInstances {
    type Item = HierarchicalDeterministicFactorInstance;
    type IntoIter = <IndexSet<HierarchicalDeterministicFactorInstance> as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.factor_instances().into_iter()
    }
}

impl FromIterator<HierarchicalDeterministicFactorInstance> for FactorInstances {
    fn from_iter<
        I: IntoIterator<Item = HierarchicalDeterministicFactorInstance>,
    >(
        iter: I,
    ) -> Self {
        Self::new(iter.into_iter().collect())
    }
}

impl FactorInstances {
    pub fn new(
        factor_instances: IndexSet<HierarchicalDeterministicFactorInstance>,
    ) -> Self {
        Self {
            __hidden: HiddenConstructor,
            factor_instances,
        }
    }

    pub fn just(
        factor_instance: HierarchicalDeterministicFactorInstance,
    ) -> Self {
        Self::new(IndexSet::just(factor_instance))
    }

    pub fn factor_instances(
        &self,
    ) -> IndexSet<HierarchicalDeterministicFactorInstance> {
        self.factor_instances.clone()
    }
}

impl HasSampleValues for FactorInstances {
    fn sample() -> Self {
        Self::new(IndexSet::from_iter([
            HierarchicalDeterministicFactorInstance::sample_mainnet_account_device_factor_fs_0_securified_at_index(0),
            HierarchicalDeterministicFactorInstance::sample_mainnet_account_device_factor_fs_0_securified_at_index(1),
        ]))
    }

    fn sample_other() -> Self {
        Self::new(IndexSet::from_iter([
            HierarchicalDeterministicFactorInstance::sample_mainnet_account_device_factor_fs_1_securified_at_index(2),
            HierarchicalDeterministicFactorInstance::sample_mainnet_account_device_factor_fs_1_securified_at_index(3),
        ]))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = FactorInstances;

    #[test]
    fn equality() {
        assert_eq!(SUT::sample(), SUT::sample());
        assert_eq!(SUT::sample_other(), SUT::sample_other());
    }

    #[test]
    fn inequality() {
        assert_ne!(SUT::sample(), SUT::sample_other());
    }

    #[test]
    fn json() {
        let element = HierarchicalDeterministicFactorInstance::sample();
        let sut = SUT::from_iter([element]);

        assert_eq_after_json_roundtrip(
            &sut,
            r#"
            [
                {
			    	"badge": {
			    		"virtualSource": {
			    			"hierarchicalDeterministicPublicKey": {
			    				"publicKey": {
			    					"curve": "curve25519",
			    					"compressedData": "c05f9fa53f203a01cbe43e89086cae29f6c7cdd5a435daa9e52b69e656739b36"
			    				},
			    				"derivationPath": {
			    					"scheme": "cap26",
			    					"path": "m/44H/1022H/1H/525H/1460H/0H"
			    				}
			    			},
			    			"discriminator": "hierarchicalDeterministicPublicKey"
			    		},
			    		"discriminator": "virtualSource"
			    	},
			    	"factorSourceID": {
			    		"fromHash": {
			    			"kind": "device",
			    			"body": "f1a93d324dd0f2bff89963ab81ed6e0c2ee7e18c0827dc1d3576b2d9f26bbd0a"
			    		},
			    		"discriminator": "fromHash"
			    	}
			    }
            ]
            "#,
        );
    }
}
