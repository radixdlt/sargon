
// #[cfg(test)]
// mod tests {
//     use radix_transactions::manifest::{AssertWorktopContains, DropAllProofs};
//
//     use super::*;
//
//     #[test]
//     fn default_dev_fee_is() {
//         assert_eq!(default_fee(), 25.into());
//     }
//
//     #[test]
//     fn is_lock_fee() {
//         assert!(!ScryptoInstruction::DropAllProofs(DropAllProofs).is_lock_fee());
//     }
//
//     #[test]
//     fn add_guarantees_divisibility_rounding() {
//         let instructions_string = r#"
// CALL_METHOD
//     Address("account_rdx128y6j78mt0aqv6372evz28hrxp8mn06ccddkr7xppc88hyvynvjdwr")
//     "lock_fee"
//     Decimal("0.61")
// ;
// CALL_METHOD
//     Address("account_rdx128y6j78mt0aqv6372evz28hrxp8mn06ccddkr7xppc88hyvynvjdwr")
//     "withdraw"
//     Address("resource_rdx1t4dy69k6s0gv040xa64cyadyefwtett62ng6xfdnljyydnml7t6g3j")
//     Decimal("0.12344")
// ;
// TAKE_FROM_WORKTOP
//     Address("resource_rdx1t4dy69k6s0gv040xa64cyadyefwtett62ng6xfdnljyydnml7t6g3j")
//     Decimal("0.12344")
//     Bucket("bucket1")
// ;
// CALL_METHOD
//     Address("account_rdx12xkzynhzgtpnnd02tudw2els2g9xl73yk54ppw8xekt2sdrlaer264")
//     "try_deposit_or_abort"
//     Bucket("bucket1")
//     Enum<0u8>()
// ;
// "#;
//         let index = 2;
//         let resource = ResourceAddress::sample_mainnet_candy();
//         let added_guaranteed_amount: Decimal = "0.12344".parse().unwrap();
//         let percentage: Decimal = "0.95".parse().unwrap();
//         let divisibility = 4;
//         let rounded_guaranteed_amount: Decimal = "0.1234".parse().unwrap();
//         assert_eq!(
//             added_guaranteed_amount.clone().round(divisibility),
//             rounded_guaranteed_amount.clone()
//         );
//         let mut manifest = TransactionManifest::new(
//             instructions_string,
//             NetworkID::Mainnet,
//             Blobs::default(),
//         )
//         .unwrap();
//         manifest = manifest
//             .modify_add_guarantees([TransactionGuarantee::new(
//                 added_guaranteed_amount,
//                 percentage,
//                 index,
//                 resource,
//                 divisibility,
//             )])
//             .unwrap();
//         let instructions = manifest.instructions().to_owned();
//         let instruction = instructions[index as usize + 1].clone();
//         assert_eq!(
//             instruction,
//             ScryptoInstruction::AssertWorktopContains(AssertWorktopContains {
//                 resource_address: resource.into(),
//                 amount: rounded_guaranteed_amount.into(),
//             })
//         );
//     }
//
//     #[test]
//     #[should_panic(
//         expected = "Expected single instruction. You MUST NOT chain calls with the manifest builder."
//     )]
//     fn single_when_more_than_one_panic() {
//         _ = single_instruction(|b| b.drop_all_proofs().drop_auth_zone_proofs())
//     }
//
//     #[test]
//     #[should_panic(
//         expected = "Expected single instruction. You MUST NOT chain calls with the manifest builder."
//     )]
//     fn single_v2_when_more_than_one_panic() {
//         _ = single_instruction_v2(|b| {
//             b.drop_all_proofs().drop_auth_zone_proofs()
//         })
//     }
//
//     #[test]
//     fn test_modify_manifest_lock_fee() {
//         let instructions_string = r#"
// CALL_METHOD
//     Address("account_rdx128dtethfy8ujrsfdztemyjk0kvhnah6dafr57frz85dcw2c8z0td87")
//     "withdraw"
//     Address("resource_rdx1tknxxxxxxxxxradxrdxxxxxxxxx009923554798xxxxxxxxxradxrd")
//     Decimal("1337")
// ;
// TAKE_FROM_WORKTOP
//     Address("resource_rdx1tknxxxxxxxxxradxrdxxxxxxxxx009923554798xxxxxxxxxradxrd")
//     Decimal("1337")
//     Bucket("bucket1")
// ;
// CALL_METHOD
//     Address("account_rdx12y02nen8zjrq0k0nku98shjq7n05kvl3j9m5d3a6cpduqwzgmenjq7")
//     "try_deposit_or_abort"
//     Bucket("bucket1")
//     Enum<0u8>()
// ;
//         "#;
//
//         let manifest = TransactionManifest::new(
//             instructions_string,
//             NetworkID::Mainnet,
//             Blobs::default(),
//         )
//         .unwrap();
//
//         manifest_eq(
//             manifest.modify_add_lock_fee(
//                 LockFeeData::new_with_fee(
//                     "account_rdx12xkzynhzgtpnnd02tudw2els2g9xl73yk54ppw8xekt2sdrlaer264".parse().unwrap(),
//                     42.into(),
//                 ),
//                 IndexMap::new()
//             ).unwrap(),
//             r#"
//         CALL_METHOD
//             Address("account_rdx12xkzynhzgtpnnd02tudw2els2g9xl73yk54ppw8xekt2sdrlaer264")
//             "lock_fee"
//             Decimal("42")
//         ;
//         CALL_METHOD
//             Address("account_rdx128dtethfy8ujrsfdztemyjk0kvhnah6dafr57frz85dcw2c8z0td87")
//             "withdraw"
//             Address("resource_rdx1tknxxxxxxxxxradxrdxxxxxxxxx009923554798xxxxxxxxxradxrd")
//             Decimal("1337")
//         ;
//         TAKE_FROM_WORKTOP
//             Address("resource_rdx1tknxxxxxxxxxradxrdxxxxxxxxx009923554798xxxxxxxxxradxrd")
//             Decimal("1337")
//             Bucket("bucket1")
//         ;
//         CALL_METHOD
//             Address("account_rdx12y02nen8zjrq0k0nku98shjq7n05kvl3j9m5d3a6cpduqwzgmenjq7")
//             "try_deposit_or_abort"
//             Bucket("bucket1")
//             Enum<0u8>()
//         ;
//             "#,
//         );
//     }
//
//     #[test]
//     fn test_modify_manifest_lock_fee_default_added_if_none_provided() {
//         let manifest = TransactionManifest::sample_mainnet_without_lock_fee();
//
//         manifest_eq(
//         manifest.modify_add_lock_fee(
//             LockFeeData::new_with_fee_payer(
//                 "account_rdx12xkzynhzgtpnnd02tudw2els2g9xl73yk54ppw8xekt2sdrlaer264".parse().unwrap(),
//             ),
//             IndexMap::new()
//         ).unwrap(),
//         r#"
//         CALL_METHOD
//             Address("account_rdx12xkzynhzgtpnnd02tudw2els2g9xl73yk54ppw8xekt2sdrlaer264")
//             "lock_fee"
//             Decimal("25")
//         ;
//         CALL_METHOD
//             Address("account_rdx128dtethfy8ujrsfdztemyjk0kvhnah6dafr57frz85dcw2c8z0td87")
//             "withdraw"
//             Address("resource_rdx1tknxxxxxxxxxradxrdxxxxxxxxx009923554798xxxxxxxxxradxrd")
//             Decimal("1337")
//         ;
//         TAKE_FROM_WORKTOP
//             Address("resource_rdx1tknxxxxxxxxxradxrdxxxxxxxxx009923554798xxxxxxxxxradxrd")
//             Decimal("1337")
//             Bucket("bucket1")
//         ;
//         CALL_METHOD
//             Address("account_rdx12y02nen8zjrq0k0nku98shjq7n05kvl3j9m5d3a6cpduqwzgmenjq7")
//             "try_deposit_or_abort"
//             Bucket("bucket1")
//             Enum<0u8>()
//         ;
//         "#,
//         );
//     }
//
//     #[test]
//     fn test_modify_manifest_add_guarantees_to_manifest_without_lock_fee() {
//         let manifest = TransactionManifest::sample_mainnet_without_lock_fee();
//
//         manifest_eq(
//             manifest
//                 .modify_add_guarantees([TransactionGuarantee::new(
//                     1337,
//                     0,
//                     1,
//                     ResourceAddress::sample(),
//                     10,
//                 )])
//                 .unwrap(),
//             r#"
//             CALL_METHOD
//                 Address("account_rdx128dtethfy8ujrsfdztemyjk0kvhnah6dafr57frz85dcw2c8z0td87")
//                 "withdraw"
//                 Address("resource_rdx1tknxxxxxxxxxradxrdxxxxxxxxx009923554798xxxxxxxxxradxrd")
//                 Decimal("1337")
//             ;
//             ASSERT_WORKTOP_CONTAINS
//                 Address("resource_rdx1tknxxxxxxxxxradxrdxxxxxxxxx009923554798xxxxxxxxxradxrd")
//                 Decimal("1337")
//             ;
//             TAKE_FROM_WORKTOP
//                 Address("resource_rdx1tknxxxxxxxxxradxrdxxxxxxxxx009923554798xxxxxxxxxradxrd")
//                 Decimal("1337")
//                 Bucket("bucket1")
//             ;
//             CALL_METHOD
//                 Address("account_rdx12y02nen8zjrq0k0nku98shjq7n05kvl3j9m5d3a6cpduqwzgmenjq7")
//                 "try_deposit_or_abort"
//                 Bucket("bucket1")
//                 Enum<0u8>()
//             ;
//             "#,
//         );
//     }
//
//     #[test]
//     fn test_modify_manifest_add_many_guarantees() {
//         let manifest = TransactionManifest::sample_mainnet_without_lock_fee();
//
//         manifest_eq(
//             manifest
//                 .modify_add_guarantees([
//                     TransactionGuarantee::new(
//                         1337,
//                         0,
//                         1,
//                         ResourceAddress::sample(),
//                         10,
//                     ),
//                     TransactionGuarantee::new(
//                         1338,
//                         0,
//                         1,
//                         ResourceAddress::sample(),
//                         10,
//                     ),
//                     TransactionGuarantee::new(
//                         1339,
//                         0,
//                         1,
//                         ResourceAddress::sample(),
//                         10,
//                     ),
//                     TransactionGuarantee::new(
//                         1340,
//                         0,
//                         1,
//                         ResourceAddress::sample(),
//                         10,
//                     ),
//                     TransactionGuarantee::new(
//                         1341,
//                         0,
//                         1,
//                         ResourceAddress::sample(),
//                         10,
//                     ),
//                     TransactionGuarantee::new(
//                         1342,
//                         0,
//                         1,
//                         ResourceAddress::sample(),
//                         10,
//                     ),
//                 ])
//                 .unwrap(),
//             r#"
//             CALL_METHOD
//                 Address("account_rdx128dtethfy8ujrsfdztemyjk0kvhnah6dafr57frz85dcw2c8z0td87")
//                 "withdraw"
//                 Address("resource_rdx1tknxxxxxxxxxradxrdxxxxxxxxx009923554798xxxxxxxxxradxrd")
//                 Decimal("1337")
//             ;
//             ASSERT_WORKTOP_CONTAINS
//                 Address("resource_rdx1tknxxxxxxxxxradxrdxxxxxxxxx009923554798xxxxxxxxxradxrd")
//                 Decimal("1337")
//             ;
//             ASSERT_WORKTOP_CONTAINS
//                 Address("resource_rdx1tknxxxxxxxxxradxrdxxxxxxxxx009923554798xxxxxxxxxradxrd")
//                 Decimal("1338")
//             ;
//             ASSERT_WORKTOP_CONTAINS
//                 Address("resource_rdx1tknxxxxxxxxxradxrdxxxxxxxxx009923554798xxxxxxxxxradxrd")
//                 Decimal("1339")
//             ;
//             ASSERT_WORKTOP_CONTAINS
//                 Address("resource_rdx1tknxxxxxxxxxradxrdxxxxxxxxx009923554798xxxxxxxxxradxrd")
//                 Decimal("1340")
//             ;
//             ASSERT_WORKTOP_CONTAINS
//                 Address("resource_rdx1tknxxxxxxxxxradxrdxxxxxxxxx009923554798xxxxxxxxxradxrd")
//                 Decimal("1341")
//             ;
//             ASSERT_WORKTOP_CONTAINS
//                 Address("resource_rdx1tknxxxxxxxxxradxrdxxxxxxxxx009923554798xxxxxxxxxradxrd")
//                 Decimal("1342")
//             ;
//             TAKE_FROM_WORKTOP
//                 Address("resource_rdx1tknxxxxxxxxxradxrdxxxxxxxxx009923554798xxxxxxxxxradxrd")
//                 Decimal("1337")
//                 Bucket("bucket1")
//             ;
//             CALL_METHOD
//                 Address("account_rdx12y02nen8zjrq0k0nku98shjq7n05kvl3j9m5d3a6cpduqwzgmenjq7")
//                 "try_deposit_or_abort"
//                 Bucket("bucket1")
//                 Enum<0u8>()
//             ;
//             "#,
//         );
//     }
//
//     #[test]
//     fn test_modify_manifest_add_guarantees_to_manifest_with_lock_fee() {
//         let manifest = TransactionManifest::sample();
//
//         manifest_eq(
//             manifest
//                 .modify_add_guarantees([TransactionGuarantee::new(
//                     1337,
//                     0,
//                     1,
//                     ResourceAddress::sample(),
//                     10,
//                 )])
//                 .unwrap(),
//             r#"
//             CALL_METHOD
//                 Address("account_rdx128y6j78mt0aqv6372evz28hrxp8mn06ccddkr7xppc88hyvynvjdwr")
//                 "lock_fee"
//                 Decimal("0.61")
//             ;
//             CALL_METHOD
//                 Address("account_rdx128y6j78mt0aqv6372evz28hrxp8mn06ccddkr7xppc88hyvynvjdwr")
//                 "withdraw"
//                 Address("resource_rdx1tknxxxxxxxxxradxrdxxxxxxxxx009923554798xxxxxxxxxradxrd")
//                 Decimal("1337")
//             ;
//             ASSERT_WORKTOP_CONTAINS
//                 Address("resource_rdx1tknxxxxxxxxxradxrdxxxxxxxxx009923554798xxxxxxxxxradxrd")
//                 Decimal("1337")
//             ;
//             TAKE_FROM_WORKTOP
//                 Address("resource_rdx1tknxxxxxxxxxradxrdxxxxxxxxx009923554798xxxxxxxxxradxrd")
//                 Decimal("1337")
//                 Bucket("bucket1")
//             ;
//             CALL_METHOD
//                 Address("account_rdx12xkzynhzgtpnnd02tudw2els2g9xl73yk54ppw8xekt2sdrlaer264")
//                 "try_deposit_or_abort"
//                 Bucket("bucket1")
//                 Enum<0u8>()
//             ;
//             "#,
//         );
//     }
//
//     #[test]
//     fn test_modify_manifest_add_guarantees_unchanged_if_no_guarantees() {
//         let manifest = TransactionManifest::sample();
//         assert_eq!(
//             manifest.clone().modify_add_guarantees([]).unwrap(),
//             manifest
//         );
//     }
//
//     #[test]
//     fn test_modify_manifest_add_guarantees_unchanged_if_instructions_empty() {
//         let manifest = TransactionManifest::empty(NetworkID::Mainnet);
//         assert_eq!(
//             manifest.clone().modify_add_guarantees([]).unwrap(),
//             manifest
//         );
//     }
//
//     #[test]
//     fn test_modify_manifest_add_guarantees_returns_error_index_equal_to_instruction_count(
//     ) {
//         let manifest = TransactionManifest::sample();
//         assert_eq!(
//             manifest.modify_add_guarantees([TransactionGuarantee::new(
//                 0,
//                 0,
//                 4,
//                 ResourceAddress::sample(),
//                 None
//             )]),
//             Err(CommonError::TXGuaranteeIndexOutOfBounds {
//                 index: 4,
//                 count: 4
//             })
//         );
//     }
//
//     #[test]
//     fn test_modify_manifest_add_guarantees_returns_error_index_larger_than_instruction_count(
//     ) {
//         let manifest = TransactionManifest::sample();
//         assert_eq!(
//             manifest.modify_add_guarantees(vec![TransactionGuarantee::new(
//                 0,
//                 0,
//                 5,
//                 ResourceAddress::sample(),
//                 None
//             )]),
//             Err(CommonError::TXGuaranteeIndexOutOfBounds {
//                 index: 5,
//                 count: 4
//             })
//         );
//     }
//
//     #[test]
//     fn test_modify_add_proofs_and_lock_fee_when_no_proofs_provided() {
//         let manifest = TransactionManifest::sample_mainnet_without_lock_fee();
//
//         manifest_eq(
//             manifest.modify_add_proofs_and_lock_fee(
//                 Some(LockFeeData::new_with_fee_payer(
//                     AccountAddress::try_from_bech32(
//                         "account_rdx12xkzynhzgtpnnd02tudw2els2g9xl73yk54ppw8xekt2sdrlaer264"
//                     ).unwrap()
//                 )),
//                 radix_rust::indexmap!()
//             ).unwrap(),
//             r#"
//         CALL_METHOD
//             Address("account_rdx12xkzynhzgtpnnd02tudw2els2g9xl73yk54ppw8xekt2sdrlaer264")
//             "lock_fee"
//             Decimal("25")
//         ;
//         CALL_METHOD
//             Address("account_rdx128dtethfy8ujrsfdztemyjk0kvhnah6dafr57frz85dcw2c8z0td87")
//             "withdraw"
//             Address("resource_rdx1tknxxxxxxxxxradxrdxxxxxxxxx009923554798xxxxxxxxxradxrd")
//             Decimal("1337")
//         ;
//         TAKE_FROM_WORKTOP
//             Address("resource_rdx1tknxxxxxxxxxradxrdxxxxxxxxx009923554798xxxxxxxxxradxrd")
//             Decimal("1337")
//             Bucket("bucket1")
//         ;
//         CALL_METHOD
//             Address("account_rdx12y02nen8zjrq0k0nku98shjq7n05kvl3j9m5d3a6cpduqwzgmenjq7")
//             "try_deposit_or_abort"
//             Bucket("bucket1")
//             Enum<0u8>()
//         ;
//         "#,
//         );
//     }
//
//     #[test]
//     fn test_modify_add_proofs_and_lock_fee_when_proofs_provided() {
//         let manifest = TransactionManifest::sample_mainnet_without_lock_fee();
//
//         // This account will lock the fee, but is also securified...
//         let account_address = AccountAddress::try_from_bech32(
//             "account_rdx12xkzynhzgtpnnd02tudw2els2g9xl73yk54ppw8xekt2sdrlaer264"
//         ).unwrap();
//         // ...by this access controller
//         let acc1 = AccessControllerAddress::try_from_bech32(
//             "accesscontroller_rdx1c0duj4lq0dc3cpl8qd420fpn5eckh8ljeysvjm894lyl5ja5yq6y5a"
//         ).unwrap();
//
//         let acc2 = AccessControllerAddress::try_from_bech32(
//             "accesscontroller_rdx1cw68j9ca4fye09mz3hshp4qydjnxhsahm68hvmz9cjhftcz9gnn375"
//         ).unwrap();
//         let acc3 = AccessControllerAddress::try_from_bech32(
//             "accesscontroller_rdx1cwdxqrwpx9hrng5e6l6qfyhp9wfem03ls9z9kvutqpmfk665pqa3y5"
//         ).unwrap();
//
//         manifest_eq(
//             manifest
//                 .modify_add_proofs_and_lock_fee(
//                     Some(LockFeeData::new_with_fee_payer(account_address)),
//                     IndexMap::from([
//                         (
//                             AddressOfAccountOrPersona::Account(account_address),
//                             acc1,
//                         ),
//                         (AddressOfAccountOrPersona::sample_mainnet(), acc2),
//                         (
//                             AddressOfAccountOrPersona::sample_mainnet_other(),
//                             acc3,
//                         ),
//                     ]),
//                 )
//                 .unwrap(),
//             r#"
//         CALL_METHOD
//             Address("accesscontroller_rdx1c0duj4lq0dc3cpl8qd420fpn5eckh8ljeysvjm894lyl5ja5yq6y5a")
//             "create_proof"
//         ;
//         CALL_METHOD
//             Address("account_rdx12xkzynhzgtpnnd02tudw2els2g9xl73yk54ppw8xekt2sdrlaer264")
//             "lock_fee"
//             Decimal("25")
//         ;
//         CALL_METHOD
//             Address("accesscontroller_rdx1cw68j9ca4fye09mz3hshp4qydjnxhsahm68hvmz9cjhftcz9gnn375")
//             "create_proof"
//         ;
//         CALL_METHOD
//             Address("accesscontroller_rdx1cwdxqrwpx9hrng5e6l6qfyhp9wfem03ls9z9kvutqpmfk665pqa3y5")
//             "create_proof"
//         ;
//         CALL_METHOD
//             Address("account_rdx128dtethfy8ujrsfdztemyjk0kvhnah6dafr57frz85dcw2c8z0td87")
//             "withdraw"
//             Address("resource_rdx1tknxxxxxxxxxradxrdxxxxxxxxx009923554798xxxxxxxxxradxrd")
//             Decimal("1337")
//         ;
//         TAKE_FROM_WORKTOP
//             Address("resource_rdx1tknxxxxxxxxxradxrdxxxxxxxxx009923554798xxxxxxxxxradxrd")
//             Decimal("1337")
//             Bucket("bucket1")
//         ;
//         CALL_METHOD
//             Address("account_rdx12y02nen8zjrq0k0nku98shjq7n05kvl3j9m5d3a6cpduqwzgmenjq7")
//             "try_deposit_or_abort"
//             Bucket("bucket1")
//             Enum<0u8>()
//         ;
//         "#,
//         );
//     }
// }
