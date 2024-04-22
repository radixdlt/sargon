use super::interaction_items::DappToWalletInteractionItems;
use super::interaction_metadata::DappToWalletInteractionMetadata;
use crate::prelude::*;

#[derive(Debug, Deserialize, PartialEq, uniffi::Record)]
#[serde(rename_all = "camelCase")]
pub struct DappToWalletInteraction {
    pub interaction_id: WalletInteractionId,
    pub items: DappToWalletInteractionItems,
    pub metadata: DappToWalletInteractionMetadata,
}


#[derive(Debug, Deserialize, PartialEq, uniffi::Record)]
#[serde(rename_all = "camelCase")]
pub struct DappToWalletInteractionUnvalidated {
    pub interaction_id: WalletInteractionId,
    pub items: DappToWalletInteractionItems,
    pub metadata: DappToWalletInteractionMetadataUnvalidated,
}

impl DappToWalletInteraction {
  pub fn from_json_bag_of_bytes(json: impl AsRef<[u8]>) -> Result<Self> {
    let json = json.as_ref();
    serde_json::from_slice::<DappToWalletInteraction>(json)
.map_err(|e| {
  error!("Failed to deserialize JSON as EncryptedProfileSnapshot, error: {:?}", e);
        CommonError::FailedToDeserializeJSONToValue {
            json_byte_count: json.len() as u64,
            type_name: "EncryptedProfileSnapshot".to_owned(),
        }})
  }
}

impl HasSampleValues for DappToWalletInteraction {
    fn sample() -> Self {
        Self {
            interaction_id: WalletInteractionId::sample(),
            items: DappToWalletInteractionItems::sample(),
            metadata: DappToWalletInteractionMetadata::sample(),
        }
    }

    fn sample_other() -> Self {
        Self {
            interaction_id: WalletInteractionId::sample_other(),
            items: DappToWalletInteractionItems::sample_other(),
            metadata: DappToWalletInteractionMetadata::sample_other(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    /*
        {
      "items" : {
        "ongoingPersonaData" : {
          "isRequestingName" : true,
          "numberOfRequestedEmailAddresses" : {
            "quantifier" : "exactly",
            "quantity" : 1
          },
          "numberOfRequestedPhoneNumbers" : {
            "quantifier" : "exactly",
            "quantity" : 1
          }
        },
        "reset" : {
          "accounts" : true,
          "personaData" : true
        },
        "discriminator" : "authorizedRequest",
        "auth" : {
          "discriminator" : "loginWithoutChallenge"
        },
        "ongoingAccounts" : {
          "challenge" : "5ec2c8da751d333cce4b020867a40b6a4aefbf9486e6e94691c0d9f84ba4dfd4",
          "numberOfAccounts" : {
            "quantifier" : "exactly",
            "quantity" : 3
          }
        }
      },
      "interactionId" : "5baf6d50-bce2-438c-9338-5ea0327e57ce",
      "metadata" : {
        "networkId" : 2,
        "dAppDefinitionAddress" : "account_tdx_2_12xd46c22d6m696lv565t9afn088htudtq275px3qs925ywwty8axze",
        "origin" : "https://dev-sandbox.rdx-works-main.extratools.works",
        "version" : 2
      }
    }
         */
    #[test]
    fn test_deserialize() {
        let json = json!({
            "interactionId": "5baf6d50-bce2-438c-9338-5ea0327e57ce",
            "items": {
                "discriminator": "authorizedRequest",
                "auth": {
                    "discriminator": "loginWithoutChallenge"
                },
                "reset": {
                    "accounts": true,
                    "personaData": true
                },
                "ongoingAccounts": {
                    "challenge": "5ec2c8da751d333cce4b020867a40b6a4aefbf9486e6e94691c0d9f84ba4dfd4",
                    "numberOfAccounts": {
                        "quantifier": "exactly",
                        "quantity": 3
                    }
                },
                "ongoingPersonaData": {
                    "isRequestingName": true,
                    "numberOfRequestedEmailAddresses": {
                        "quantifier": "exactly",
                        "quantity": 1
                    },
                    "numberOfRequestedPhoneNumbers": {
                        "quantifier": "exactly",
                        "quantity": 1
                    }
                }
            },
            "metadata": {
                "version": 2,
                "networkId": 2,
                "origin": "https://dev-sandbox.rdx-works-main.extratools.works",
                "dAppDefinitionAddress": "account_tdx_2_12xd46c22d6m696lv565t9afn088htudtq275px3qs925ywwty8axze"
            }
        });
    }
}
