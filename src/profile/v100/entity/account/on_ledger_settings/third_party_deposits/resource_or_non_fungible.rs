use crate::prelude::*;

/// The addresses that can be added as exception to the `DepositRule`
#[derive(
    Serialize,
    Deserialize,
    Clone,
    Debug,
    PartialEq,
    Eq,
    Hash,
    uniffi::Enum,
)]
#[serde(tag = "discriminator")]
pub enum ResourceOrNonFungible {
    #[serde(rename = "resourceAddress")]
    Resource { value: ResourceAddress },
    
    #[serde(rename = "nonFungibleGlobalID")]
    NonFungible { value: NonFungibleGlobalId },
}

#[cfg(test)]
mod tests {
    use crate::prelude::*;
    #[test]
    fn json_decode_deny_all_with_exceptions() {
        let model =
            ResourceOrNonFungible::Resource {
                value: "resource_rdx1tkk83magp3gjyxrpskfsqwkg4g949rmcjee4tu2xmw93ltw2cz94sq"
                    .parse()
                    .unwrap(),
            };

        assert_eq_after_json_roundtrip(
            &model,
            r#"
            {
              "value" : "resource_rdx1tkk83magp3gjyxrpskfsqwkg4g949rmcjee4tu2xmw93ltw2cz94sq",
              "discriminator" : "resourceAddress"
            }
            "#,
        )
    }
}
