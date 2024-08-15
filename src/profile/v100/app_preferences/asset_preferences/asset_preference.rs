use crate::prelude::*;

#[derive(
    Deserialize,
    Serialize,
    Clone,
    PartialEq,
    Default,
    Eq,
    Debug,
    Hash,
    uniffi::Record,
)]
pub struct AssetPreference {
    pub visibility: AssetVisibility,
}

impl AssetPreference {
    pub fn set_visibility(&mut self, visibility: AssetVisibility) {
        self.visibility = visibility;
    }
}

impl HasSampleValues for AssetPreference {
    fn sample() -> Self {
        Self {
            visibility: AssetVisibility::sample(),
        }
    }

    fn sample_other() -> Self {
        Self {
            visibility: AssetVisibility::sample_other(),
        }
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

        sut.set_visibility(AssetVisibility::Visible);
        assert_eq!(AssetVisibility::Visible, sut.visibility);
    }

    #[test]
    fn json_roundtrip() {
        let sut = SUT::sample();
        assert_eq_after_json_roundtrip(
            &sut,
            r#"
            {
                "visibility": "hidden"
            }
            "#,
        );
    }
}