use crate::prelude::*;
use radix_transactions::builder::SubintentManifestV2Builder;

impl ModifyingManifest<SubintentManifest, ScryptoInstructionV2>
    for SubintentManifest
{
    fn modifying_manifest(&self) -> SubintentManifest {
        self.clone()
    }
}

impl AddingGuaranteesModifyingManifest<SubintentManifest, ScryptoInstructionV2>
    for SubintentManifest
{
    fn insert_guarantee_assertion_at_position(
        self,
        position: InstructionPosition,
        guarantee: TransactionGuarantee,
    ) -> Result<Self> {
        let rounded_amount = guarantee.rounded_amount();

        let instruction = single_instruction_v2(|b| {
            b.assert_worktop_contains(
                &guarantee.resource_address,
                rounded_amount,
            )
        });

        let mut instructions = self.instructions().clone();
        instructions.insert(position.0 as usize, instruction);

        let instructions = InstructionsV2::try_from((
            instructions.as_ref(),
            self.network_id(),
        ))?;

        Ok(SubintentManifest::with_instructions_and_blobs_and_children(
            instructions,
            self.blobs().clone(),
            self.children().clone(),
        ))
    }
}

impl
    AddingLockFeeAndProofsModifyingManifest<
        SubintentManifest,
        SubintentManifestV2Builder,
        ScryptoInstructionV2,
    > for SubintentManifest
{
}

// #[cfg(test)]
// mod test {
//     use super::*;
//
//     #[test]
//     fn test_modify() {
//         let manifest = SubintentManifest::sample();
//
//         let dapp_fee_payer_address = AccountAddress::try_from_bech32(
//             "account_rdx128y6j78mt0aqv6372evz28hrxp8mn06ccddkr7xppc88hyvynvjdwr"
//         ).unwrap();
//
//         let securified_entities = IndexMap::from([
//             (
//                 AddressOfAccountOrPersona::from(dapp_fee_payer_address),
//                 AccessControllerAddress::sample_mainnet(),
//             ),
//             (
//                 AddressOfAccountOrPersona::sample_mainnet_other(),
//                 AccessControllerAddress::sample_mainnet_other(),
//             ),
//         ]);
//
//         subintent_manifest_eq(
//             manifest
//                 .modify(
//                     securified_entities,
//                     [TransactionGuarantee::new(
//                         1337,
//                         0,
//                         3,
//                         ResourceAddress::sample(),
//                         10,
//                     )],
//                 )
//                 .unwrap(),
//             r#"
//             CALL_METHOD
//                 Address("accesscontroller_rdx1c0duj4lq0dc3cpl8qd420fpn5eckh8ljeysvjm894lyl5ja5yq6y5a")
//                 "create_proof"
//             ;
//             CALL_METHOD
//                 Address("accesscontroller_rdx1cv93xuha64eay8ctkx9km0el2jgkuh6gqlwec7tzecccyu0rj37xak")
//                 "create_proof"
//             ;
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
//             YIELD_TO_PARENT;
//             "#,
//         );
//     }
// }
