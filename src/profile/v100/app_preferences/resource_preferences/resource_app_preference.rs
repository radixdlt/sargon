use crate::prelude::*;

/// A preference the user has configured off-ledger for a given resource.
/// Allows users, for example, to hide a given resource on their accounts.
///
/// Named like this to differ from RET's `ResourcePreference`.
#[derive(
    Deserialize, Serialize, Clone, PartialEq, Eq, Debug, Hash, uniffi::Record,
)]
pub struct ResourceAppPreference {
    pub resource: ResourceIdentifier,
    pub visibility: ResourceVisibility,
}

impl ResourceAppPreference {
    pub fn new(
        resource: impl Into<ResourceIdentifier>,
        visibility: ResourceVisibility,
    ) -> Self {
        Self {
            resource: resource.into(),
            visibility,
        }
    }
}

impl Identifiable for ResourceAppPreference {
    type ID = ResourceIdentifier;
    fn id(&self) -> Self::ID {
        self.resource.clone()
    }
}

impl HasSampleValues for ResourceAppPreference {
    fn sample() -> Self {
        Self::new(ResourceIdentifier::sample(), ResourceVisibility::sample())
    }

    fn sample_other() -> Self {
        Self::new(
            ResourceIdentifier::sample_other(),
            ResourceVisibility::sample_other(),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = ResourceAppPreference;

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
    fn visibility() {
        let mut sut = SUT::sample();
        assert_eq!(ResourceVisibility::Hidden, sut.visibility);

        sut.visibility = ResourceVisibility::Visible;
        assert_eq!(ResourceVisibility::Visible, sut.visibility);
    }

    #[test]
    fn json_roundtrip() {
        let sut = SUT::sample();
        assert_eq_after_json_roundtrip(
            &sut,
            r#"
            {
                "resource": {
                    "kind": "fungible",
                    "value": "resource_rdx1tknxxxxxxxxxradxrdxxxxxxxxx009923554798xxxxxxxxxradxrd"
                },
                "visibility": "hidden"
            }
            "#,
        );
    }
}
