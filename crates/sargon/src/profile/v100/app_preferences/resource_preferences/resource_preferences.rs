use crate::prelude::*;
use core::hash::Hash;
use std::{hash::Hasher, ops::Index};

decl_identified_vec_of!(
    /// User off-ledger preferences regarding resources.
    ResourcePreferences,
    ResourceAppPreference
);

impl HasSampleValues for ResourcePreferences {
    fn sample() -> Self {
        Self::from_iter([
            ResourceAppPreference::sample(),
            ResourceAppPreference::sample_other(),
        ])
    }

    fn sample_other() -> Self {
        Self::from_iter([ResourceAppPreference::sample_other()])
    }
}

impl ResourcePreferences {
    pub fn get_hidden_resources(&self) -> HiddenResources {
        self.iter()
            .filter(|x| x.visibility == ResourceVisibility::Hidden)
            .map(|x| x.resource)
            .collect()
    }

    pub fn hide_resource(&mut self, resource: ResourceIdentifier) {
        if !self.update_with(resource.id(), |x| {
            x.visibility = ResourceVisibility::Hidden
        }) {
            let item = ResourceAppPreference::new(
                resource,
                ResourceVisibility::Hidden,
            );
            self.append(item);
        }
    }

    pub fn unhide_resource(&mut self, resource: ResourceIdentifier) {
        if !self.update_with(resource.id(), |x| {
            x.visibility = ResourceVisibility::Visible
        }) {
            let item = ResourceAppPreference::new(
                resource,
                ResourceVisibility::Visible,
            );
            self.append(item);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = ResourcePreferences;

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
    fn hidden_resources() {
        use crate::ResourceIdentifier::*;
        let mut sut = SUT::new();

        // Test with no resources hidden
        let mut result = sut.get_hidden_resources();
        assert!(result.is_empty());

        // Test unhiding an resource that wasn't present
        let pool_unit = ResourceIdentifier::PoolUnit(PoolAddress::sample());
        sut.unhide_resource(pool_unit.clone());
        result = sut.get_hidden_resources();
        assert!(result.is_empty());

        // Test with some resources hidden
        let fungible_one =
            ResourceIdentifier::Fungible(ResourceAddress::sample_other());
        let fungible_two =
            ResourceIdentifier::Fungible(ResourceAddress::sample());
        sut.hide_resource(fungible_one.clone());
        sut.hide_resource(fungible_two.clone());

        result = sut.get_hidden_resources();
        assert_eq!(
            HiddenResources::from_iter([
                fungible_one.clone(),
                fungible_two.clone()
            ]),
            result
        );

        // Test hiding some non-fungible and pool unit, and unhiding one of the fungibles
        let non_fungible =
            ResourceIdentifier::NonFungible(ResourceAddress::sample_other());
        sut.unhide_resource(fungible_one);
        sut.hide_resource(non_fungible.clone());
        sut.hide_resource(pool_unit.clone());

        result = sut.get_hidden_resources();
        assert_eq!(
            HiddenResources::from_iter([fungible_two, non_fungible, pool_unit]),
            result
        );
    }

    #[test]
    fn json_roundtrip() {
        let sut = SUT::sample();
        assert_eq_after_json_roundtrip(
            &sut,
            r#"
            [
                {
                    "resource": {
                        "kind": "fungible",
                        "value": "resource_rdx1tknxxxxxxxxxradxrdxxxxxxxxx009923554798xxxxxxxxxradxrd"
                    },
                    "visibility": "hidden"
                },
                {
                    "resource": {
                        "kind": "nonFungible",
                        "value": "resource_rdx1t4dy69k6s0gv040xa64cyadyefwtett62ng6xfdnljyydnml7t6g3j"
                    },
                    "visibility": "visible"
                }
            ]
            "#,
        );
    }
}
