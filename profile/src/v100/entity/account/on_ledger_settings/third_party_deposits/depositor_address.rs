use crate::prelude::*;

/// The addresses that can be added as exception to the `DepositRule`
#[derive(
    Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, uniffi::Enum,
)]
#[serde(rename_all = "camelCase")]
#[serde(tag = "discriminator")]
pub enum DepositorAddress {
    ResourceAddress { value: ResourceAddress },
    NonFungibleGlobalID { value: NonFungibleGlobalId },
}

#[cfg(test)]
mod tests {
    use crate::prelude::*;
    #[test]
    fn json_decode_deny_all_with_exceptions() {
        let model =
            DepositorAddress::ResourceAddress {
                value: "resource_rdx1tkk83magp3gjyxrpskfsqwkg4g949rmcjee4tu2xmw93ltw2cz94sq"
                    .try_into()
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
