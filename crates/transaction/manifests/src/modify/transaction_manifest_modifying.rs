use crate::prelude::*;

impl ModifyingManifest<TransactionManifest, ScryptoInstruction>
    for TransactionManifest
{
    fn modifying_manifest(&self) -> TransactionManifest {
        self.clone()
    }
}

impl AddingGuaranteesModifyingManifest<TransactionManifest, ScryptoInstruction>
    for TransactionManifest
{
    fn insert_guarantee_assertion_at_position(
        self,
        position: InstructionPosition,
        guarantee: TransactionGuarantee,
    ) -> Result<Self> {
        let rounded_amount = guarantee.rounded_amount();

        let instruction = single_instruction(|b| {
            b.assert_worktop_contains(
                &guarantee.resource_address,
                rounded_amount,
            )
        });

        let mut instructions = self.instructions().clone();
        instructions.insert(position.0 as usize, instruction);

        let instructions =
            Instructions::try_from((instructions.as_ref(), self.network_id()))?;

        Ok(TransactionManifest::with_instructions_and_blobs(
            instructions,
            self.blobs().clone(),
        ))
    }
}

impl
    AddingLockFeeAndProofsModifyingManifest<
        TransactionManifest,
        ScryptoTransactionManifestBuilder,
        ScryptoInstruction,
    > for TransactionManifest
{
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_modify_add_lock_fee_without_fee_amount() {
        let manifest = TransactionManifest::sample_mainnet_without_lock_fee();
        let fee_payer = AccountAddress::sample_mainnet();

        manifest_eq(
            manifest
                .modify_add_lock_fee(
                    LockFeeData::new_with_unsecurified_fee_payer_no_fee(
                        fee_payer,
                    ),
                )
                .unwrap(),
            r#"
            CALL_METHOD
                Address("account_rdx128y6j78mt0aqv6372evz28hrxp8mn06ccddkr7xppc88hyvynvjdwr")
                "lock_fee"
                Decimal("25")
            ;
            CALL_METHOD
                Address("account_rdx128dtethfy8ujrsfdztemyjk0kvhnah6dafr57frz85dcw2c8z0td87")
                "withdraw"
                Address("resource_rdx1tknxxxxxxxxxradxrdxxxxxxxxx009923554798xxxxxxxxxradxrd")
                Decimal("1337")
            ;
            TAKE_FROM_WORKTOP
                Address("resource_rdx1tknxxxxxxxxxradxrdxxxxxxxxx009923554798xxxxxxxxxradxrd")
                Decimal("1337")
                Bucket("bucket1")
            ;
            CALL_METHOD
                Address("account_rdx12y02nen8zjrq0k0nku98shjq7n05kvl3j9m5d3a6cpduqwzgmenjq7")
                "try_deposit_or_abort"
                Bucket("bucket1")
                Enum<0u8>()
            ;
            "#,
        );
    }

    #[test]
    fn test_modify_add_lock_fee_with_fee_amount() {
        let manifest = TransactionManifest::sample_mainnet_without_lock_fee();
        let fee_payer = AccountAddress::sample_mainnet();

        manifest_eq(
            manifest
                .modify_add_lock_fee(
                    LockFeeData::new_with_unsecurified_fee_payer(
                        fee_payer,
                        Decimal192::five(),
                    ),
                )
                .unwrap(),
            r#"
            CALL_METHOD
                Address("account_rdx128y6j78mt0aqv6372evz28hrxp8mn06ccddkr7xppc88hyvynvjdwr")
                "lock_fee"
                Decimal("5")
            ;
            CALL_METHOD
                Address("account_rdx128dtethfy8ujrsfdztemyjk0kvhnah6dafr57frz85dcw2c8z0td87")
                "withdraw"
                Address("resource_rdx1tknxxxxxxxxxradxrdxxxxxxxxx009923554798xxxxxxxxxradxrd")
                Decimal("1337")
            ;
            TAKE_FROM_WORKTOP
                Address("resource_rdx1tknxxxxxxxxxradxrdxxxxxxxxx009923554798xxxxxxxxxradxrd")
                Decimal("1337")
                Bucket("bucket1")
            ;
            CALL_METHOD
                Address("account_rdx12y02nen8zjrq0k0nku98shjq7n05kvl3j9m5d3a6cpduqwzgmenjq7")
                "try_deposit_or_abort"
                Bucket("bucket1")
                Enum<0u8>()
            ;
            "#,
        );
    }

    #[test]
    fn test_modify_add_lock_fee_with_fee_amount_and_access_controller() {
        let manifest = TransactionManifest::sample_mainnet_without_lock_fee();
        let fee_payer = AccountAddress::sample_mainnet();

        manifest_eq(
            manifest
                .modify_add_lock_fee(
                    LockFeeData::new_with_securified_fee_payer(
                        fee_payer,
                        AccessControllerAddress::sample_mainnet(),
                        Decimal192::five(),
                    ),
                )
                .unwrap(),
            r#"
            CALL_METHOD
                Address("accesscontroller_rdx1c0duj4lq0dc3cpl8qd420fpn5eckh8ljeysvjm894lyl5ja5yq6y5a")
                "create_proof"
            ;
            CALL_METHOD
                Address("account_rdx128y6j78mt0aqv6372evz28hrxp8mn06ccddkr7xppc88hyvynvjdwr")
                "lock_fee"
                Decimal("5")
            ;
            CALL_METHOD
                Address("account_rdx128dtethfy8ujrsfdztemyjk0kvhnah6dafr57frz85dcw2c8z0td87")
                "withdraw"
                Address("resource_rdx1tknxxxxxxxxxradxrdxxxxxxxxx009923554798xxxxxxxxxradxrd")
                Decimal("1337")
            ;
            TAKE_FROM_WORKTOP
                Address("resource_rdx1tknxxxxxxxxxradxrdxxxxxxxxx009923554798xxxxxxxxxradxrd")
                Decimal("1337")
                Bucket("bucket1")
            ;
            CALL_METHOD
                Address("account_rdx12y02nen8zjrq0k0nku98shjq7n05kvl3j9m5d3a6cpduqwzgmenjq7")
                "try_deposit_or_abort"
                Bucket("bucket1")
                Enum<0u8>()
            ;
            "#,
        );
    }

    #[test]
    fn test_modify_add_lock_fee_with_fee_amount_and_proofs() {
        let manifest = TransactionManifest::sample_mainnet_without_lock_fee();
        let fee_payer = AccountAddress::sample_mainnet();
        let proofs = IndexMap::just((
            AddressOfAccountOrPersona::sample_mainnet_other(),
            AccessControllerAddress::sample_mainnet(),
        ));

        manifest_eq(
            manifest
                .modify_add_lock_fee_and_proofs(
                    LockFeeData::new_with_unsecurified_fee_payer(
                        fee_payer,
                        Decimal192::five(),
                    ),
                    proofs,
                )
                .unwrap(),
            r#"
            CALL_METHOD
                Address("account_rdx128y6j78mt0aqv6372evz28hrxp8mn06ccddkr7xppc88hyvynvjdwr")
                "lock_fee"
                Decimal("5")
            ;
            CALL_METHOD
                Address("accesscontroller_rdx1c0duj4lq0dc3cpl8qd420fpn5eckh8ljeysvjm894lyl5ja5yq6y5a")
                "create_proof"
            ;
            CALL_METHOD
                Address("account_rdx128dtethfy8ujrsfdztemyjk0kvhnah6dafr57frz85dcw2c8z0td87")
                "withdraw"
                Address("resource_rdx1tknxxxxxxxxxradxrdxxxxxxxxx009923554798xxxxxxxxxradxrd")
                Decimal("1337")
            ;
            TAKE_FROM_WORKTOP
                Address("resource_rdx1tknxxxxxxxxxradxrdxxxxxxxxx009923554798xxxxxxxxxradxrd")
                Decimal("1337")
                Bucket("bucket1")
            ;
            CALL_METHOD
                Address("account_rdx12y02nen8zjrq0k0nku98shjq7n05kvl3j9m5d3a6cpduqwzgmenjq7")
                "try_deposit_or_abort"
                Bucket("bucket1")
                Enum<0u8>()
            ;
            "#,
        );
    }

    #[test]
    fn test_modify_add_lock_fee_with_fee_amount_and_proofs_fee_payer_in_proofs()
    {
        let manifest = TransactionManifest::sample_mainnet_without_lock_fee();
        let fee_payer = AccountAddress::sample_mainnet();
        let proofs = IndexMap::from([
            (
                AddressOfAccountOrPersona::sample_mainnet_other(),
                AccessControllerAddress::sample_mainnet(),
            ),
            (
                AddressOfAccountOrPersona::from(fee_payer),
                AccessControllerAddress::sample_mainnet_other(),
            ),
        ]);

        manifest_eq(
            manifest
                .modify_add_lock_fee_and_proofs(
                    LockFeeData::new_with_securified_fee_payer(
                        fee_payer,
                        AccessControllerAddress::sample_mainnet(),
                        Decimal192::five(),
                    ),
                    proofs,
                )
                .unwrap(),
            r#"
            CALL_METHOD
                Address("accesscontroller_rdx1c0duj4lq0dc3cpl8qd420fpn5eckh8ljeysvjm894lyl5ja5yq6y5a")
                "create_proof"
            ;
            CALL_METHOD
                Address("account_rdx128y6j78mt0aqv6372evz28hrxp8mn06ccddkr7xppc88hyvynvjdwr")
                "lock_fee"
                Decimal("5")
            ;
            CALL_METHOD
                Address("accesscontroller_rdx1cv93xuha64eay8ctkx9km0el2jgkuh6gqlwec7tzecccyu0rj37xak")
                "create_proof"
            ;
            CALL_METHOD
                Address("account_rdx128dtethfy8ujrsfdztemyjk0kvhnah6dafr57frz85dcw2c8z0td87")
                "withdraw"
                Address("resource_rdx1tknxxxxxxxxxradxrdxxxxxxxxx009923554798xxxxxxxxxradxrd")
                Decimal("1337")
            ;
            TAKE_FROM_WORKTOP
                Address("resource_rdx1tknxxxxxxxxxradxrdxxxxxxxxx009923554798xxxxxxxxxradxrd")
                Decimal("1337")
                Bucket("bucket1")
            ;
            CALL_METHOD
                Address("account_rdx12y02nen8zjrq0k0nku98shjq7n05kvl3j9m5d3a6cpduqwzgmenjq7")
                "try_deposit_or_abort"
                Bucket("bucket1")
                Enum<0u8>()
            ;
            "#,
        );
    }

    // #[test]
    // fn test_modify() {
    //     let manifest = TransactionManifest::sample_mainnet_without_lock_fee();
    //
    //     let fee_payer_address = AccountAddress::try_from_bech32(
    //         "account_rdx128dtethfy8ujrsfdztemyjk0kvhnah6dafr57frz85dcw2c8z0td87"
    //     ).unwrap();
    //
    //     let securified_entities = IndexMap::from([
    //         (
    //             AddressOfAccountOrPersona::from(fee_payer_address),
    //             AccessControllerAddress::sample_mainnet(),
    //         ),
    //         (
    //             AddressOfAccountOrPersona::sample_mainnet_other(),
    //             AccessControllerAddress::sample_mainnet_other(),
    //         ),
    //     ]);
    //
    //     manifest_eq(
    //         manifest
    //             .modify(
    //                 &fee_payer_address,
    //                 Decimal192::five(),
    //                 securified_entities,
    //                 [TransactionGuarantee::new(
    //                     1337,
    //                     0,
    //                     3,
    //                     ResourceAddress::sample(),
    //                     10,
    //                 )],
    //             )
    //             .unwrap(),
    //         r#"
    //         CALL_METHOD
    //             Address("accesscontroller_rdx1c0duj4lq0dc3cpl8qd420fpn5eckh8ljeysvjm894lyl5ja5yq6y5a")
    //             "create_proof"
    //         ;
    //         CALL_METHOD
    //             Address("account_rdx128dtethfy8ujrsfdztemyjk0kvhnah6dafr57frz85dcw2c8z0td87")
    //             "lock_fee"
    //             Decimal("5")
    //         ;
    //         CALL_METHOD
    //             Address("accesscontroller_rdx1cv93xuha64eay8ctkx9km0el2jgkuh6gqlwec7tzecccyu0rj37xak")
    //             "create_proof"
    //         ;
    //         CALL_METHOD
    //             Address("account_rdx128dtethfy8ujrsfdztemyjk0kvhnah6dafr57frz85dcw2c8z0td87")
    //             "withdraw"
    //             Address("resource_rdx1tknxxxxxxxxxradxrdxxxxxxxxx009923554798xxxxxxxxxradxrd")
    //             Decimal("1337")
    //         ;
    //         ASSERT_WORKTOP_CONTAINS
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
    //     );
    // }
}
