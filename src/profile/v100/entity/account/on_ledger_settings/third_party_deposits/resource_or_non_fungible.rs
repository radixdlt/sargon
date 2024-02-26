use crate::prelude::*;

/// The addresses that can be added as exception to the `DepositRule`
#[derive(
    Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Hash, uniffi::Enum,
)]
#[serde(tag = "discriminator")]
pub enum ResourceOrNonFungible {
    #[serde(rename = "resourceAddress")]
    Resource { value: ResourceAddress },

    #[serde(rename = "nonFungibleGlobalID")]
    NonFungible { value: NonFungibleGlobalId },
}

impl HasPlaceholder for ResourceOrNonFungible {
    fn placeholder() -> Self {
        Self::Resource {
            value: ResourceAddress::placeholder(),
        }
    }

    fn placeholder_other() -> Self {
        Self::NonFungible {
            value: NonFungibleGlobalId::placeholder(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = ResourceOrNonFungible;

    #[test]
    fn equality() {
        assert_eq!(SUT::placeholder(), SUT::placeholder());
        assert_eq!(SUT::placeholder_other(), SUT::placeholder_other());
    }

    #[test]
    fn inequality() {
        assert_ne!(SUT::placeholder(), SUT::placeholder_other());
    }

    #[test]
    fn json_roundtrip_placeholder() {
        let sut = SUT::placeholder();
        assert_eq_after_json_roundtrip(
            &sut,
            r#"
            {
              "value" : "resource_rdx1tknxxxxxxxxxradxrdxxxxxxxxx009923554798xxxxxxxxxradxrd",
              "discriminator" : "resourceAddress"
            }
            "#,
        )
    }

    #[test]
    fn json_roundtrip_placeholder_other() {
        let sut = SUT::placeholder_other();
        assert_eq_after_json_roundtrip(
            &sut,
            r#"
            {
              "value" : "resource_rdx1nfyg2f68jw7hfdlg5hzvd8ylsa7e0kjl68t5t62v3ttamtejc9wlxa:<Member_237>",
              "discriminator" : "nonFungibleGlobalID"
            }
            "#,
        )
    }
}
