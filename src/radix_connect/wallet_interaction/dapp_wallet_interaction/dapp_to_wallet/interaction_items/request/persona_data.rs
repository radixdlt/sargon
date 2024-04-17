use crate::prelude::*;
use serde::Deserialize;

#[derive(Debug, Deserialize, PartialEq, uniffi::Record)]
#[serde(rename_all = "camelCase")]
pub struct DappToWalletInteractionPersonaDataRequestItem {
    pub is_requesting_name: Option<bool>,
    pub number_of_requested_email_addresses: Option<RequestedQuantity>,
    pub number_of_requested_phone_numbers: Option<RequestedQuantity>,
}

impl HasSampleValues for DappToWalletInteractionPersonaDataRequestItem {
    fn sample() -> Self {
        Self {
            is_requesting_name: Some(true),
            number_of_requested_email_addresses: Some(
                RequestedQuantity::sample(),
            ),
            number_of_requested_phone_numbers: Some(RequestedQuantity::sample()),
        }
    }

    fn sample_other() -> Self {
        Self {
            is_requesting_name: Some(false),
            number_of_requested_email_addresses: Some(
                RequestedQuantity::sample_other(),
            ),
            number_of_requested_phone_numbers: Some(
                RequestedQuantity::sample_other(),
            ),
        }
    }
}

// impl TestVector for RequestedQuantity {
//     fn test_vectors() -> Vec<Self> {
//         let quantifiers = RequestedNumberQuantifier::test_vectors();
//         let quantities = vec![1];

//         quantifiers
//             .into_iter()
//             .cartesian_product(quantities.into_iter())
//             .map(|(quantifier, quantity)| RequestedQuantity {
//                 quantifier,
//                 quantity,
//             })
//             .collect()
//     }
// }

// impl TestVector for DappToWalletInteractionPersonaDataRequestItem {
//     fn test_vectors() -> Vec<Self> {
//         let names = Option::<bool>::test_vectors();
//         let emails = Option::<RequestedQuantity>::test_vectors();
//         let phone_numbers = Option::<RequestedQuantity>::test_vectors();

//         names
//             .into_iter()
//             .cartesian_product(emails.into_iter())
//             .cartesian_product(phone_numbers.into_iter())
//             .map(|((name, email), phone)| DappToWalletInteractionPersonaDataRequestItem {
//                 is_requesting_name: name,
//                 number_of_requested_email_addresses: email,
//                 number_of_requested_phone_numbers: phone,
//             })
//             .collect()
//     }
// }

// #[cfg(test)]
// mod tests {
//     use super::*;
//     use serde_json::json;
//     use serde_json::to_string_pretty;
//     use std::fs::File;
//     use std::io::Write;
//     use std::path::Path;

//     // Uncomment to gerenate test vectors
//    #[test]
//     fn generate_test_vectors() {
//         let vectors = DappToWalletInteractionPersonaDataRequestItem::test_vectors();
//         // Serialize the vector of combinations to JSON
//         let json = to_string_pretty(&vectors).unwrap();

//         // Define the path relative to the crate root
//         let path = Path::new("fixtures/vector/wallet_interaction/persona_data_request_item.json");
//         let mut file = File::create(&path).unwrap();

//         // Write the JSON data to the file
//         file.write_all(json.as_bytes());
//     }

//     #[test]
//     fn test_deserialize() {
//         let vectors = DappToWalletInteractionPersonaDataRequestItem::test_vectors();
//         for vector in vectors {
//             let json = json!(vector);
//             let deserialized: DappToWalletInteractionPersonaDataRequestItem = serde_json::from_value(json).unwrap();
//             assert_eq!(deserialized, vector);
//         }
//     }
// }
