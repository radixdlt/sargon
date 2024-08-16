use crate::prelude::*;

/// A preference the user has configured off-ledger for a given asset.
/// Allows users, for example, to hide a given asset on their accounts.
#[derive(
    Deserialize, Serialize, Clone, PartialEq, Eq, Debug, Hash, uniffi::Record,
)]
pub struct AssetPreference {
    pub asset_address: AssetAddress,
    pub visibility: AssetVisibility,
}

impl AssetPreference {
    pub fn new(
        asset_address: impl Into<AssetAddress>,
        visibility: AssetVisibility,
    ) -> Self {
        Self {
            asset_address: asset_address.into(),
            visibility,
        }
    }
}

impl Identifiable for AssetPreference {
    type ID = AssetAddress;
    fn id(&self) -> Self::ID {
        self.asset_address.clone()
    }
}

impl HasSampleValues for AssetPreference {
    fn sample() -> Self {
        Self::new(ResourceAddress::sample(), AssetVisibility::sample())
    }

    fn sample_other() -> Self {
        Self::new(
            NonFungibleGlobalId::sample(),
            AssetVisibility::sample_other(),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = AssetPreference;

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
        assert_eq!(AssetVisibility::Hidden, sut.visibility);

        sut.visibility = AssetVisibility::Visible;
        assert_eq!(AssetVisibility::Visible, sut.visibility);
    }

    #[test]
    fn json_roundtrip() {
        let sut = SUT::sample();
        assert_eq_after_json_roundtrip(
            &sut,
            r#"
            {
                "asset_address": {
                    "kind": "fungible",
                    "value": "resource_rdx1tknxxxxxxxxxradxrdxxxxxxxxx009923554798xxxxxxxxxradxrd"
                },
                "visibility": "hidden"
            }
            "#,
        );
    }
}
