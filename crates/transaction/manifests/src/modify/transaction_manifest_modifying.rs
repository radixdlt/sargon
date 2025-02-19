use crate::prelude::*;
use radix_common::prelude::indexmap::indexmap;

pub trait TransactionManifestModifying {
    /// Modifies the transaction manifest applying the lock fee instruction on
    /// `address_of_fee_payer` with `fee` amount
    fn modify_add_lock_fee(
        self,
        address_of_fee_payer: &AccountAddress,
        fee: impl Into<Option<Decimal192>>,
    ) -> Result<TransactionManifest>;

    /// Modifies the transaction manifest applying the following instructions
    /// - adds lock fee instruction on `address_of_fee_payer` with `fee` amount
    /// - attaches `AccessControllerAddress` proofs for `entities_with_access_controllers`, if
    ///   the entity locking fee is controller by one access controller, then the `create_proof`
    ///   for that entity is applied before `lock_fee`
    /// - adds guarantee assertions in specific indices described in `guarantees`. Those indices
    ///   will be offset by `1` if lock fee instruction is added. Remember that those indices
    ///   are received from `transaction/preview` were `lock_fee` is not present.
    fn modify<G>(
        self,
        address_of_fee_payer: &AccountAddress,
        fee: Decimal192,
        entities_with_access_controllers: IndexMap<
            AddressOfAccountOrPersona,
            AccessControllerAddress,
        >,
        guarantees: G,
    ) -> Result<TransactionManifest>
    where
        G: IntoIterator<Item = TransactionGuarantee>;
}

impl TransactionManifestModifying for TransactionManifest {
    fn modify_add_lock_fee(
        self,
        address_of_fee_payer: &AccountAddress,
        fee: impl Into<Option<Decimal192>>,
    ) -> Result<TransactionManifest> {
        let lock_fee_data = fee
            .into()
            .map(|fee| LockFeeData::new_with_fee(*address_of_fee_payer, fee))
            .unwrap_or(LockFeeData::new_with_fee_payer(*address_of_fee_payer));

        self.modify_add_proofs_and_lock_fee(Some(lock_fee_data), indexmap!())
    }

    fn modify<G>(
        self,
        address_of_fee_payer: &AccountAddress,
        fee: Decimal192,
        entities_with_access_controllers: IndexMap<
            AddressOfAccountOrPersona,
            AccessControllerAddress,
        >,
        guarantees: G,
    ) -> Result<TransactionManifest>
    where
        G: IntoIterator<Item = TransactionGuarantee>,
    {
        let modified = self.modify_add_proofs_and_lock_fee(
            Some(LockFeeData::new_with_fee(*address_of_fee_payer, fee)),
            entities_with_access_controllers,
        )?;

        modified.modify_add_guarantees(guarantees)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_modify_add_lock_fee_without_fee_amount() {
        let manifest = TransactionManifest::sample_mainnet_without_lock_fee();
        let fee_payer = AccountAddress::sample_mainnet();

        manifest_eq(
            manifest.modify_add_lock_fee(&fee_payer, None).unwrap(),
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
                .modify_add_lock_fee(&fee_payer, Decimal192::five())
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
    fn test_modify() {
        let manifest = TransactionManifest::sample_mainnet_without_lock_fee();

        let fee_payer_address = AccountAddress::try_from_bech32(
            "account_rdx128dtethfy8ujrsfdztemyjk0kvhnah6dafr57frz85dcw2c8z0td87"
        ).unwrap();

        let securified_entities = IndexMap::from([
            (
                AddressOfAccountOrPersona::from(fee_payer_address),
                AccessControllerAddress::sample_mainnet(),
            ),
            (
                AddressOfAccountOrPersona::sample_mainnet_other(),
                AccessControllerAddress::sample_mainnet_other(),
            ),
        ]);

        manifest_eq(
            manifest
                .modify(
                    &fee_payer_address,
                    Decimal192::five(),
                    securified_entities,
                    [TransactionGuarantee::new(
                        1337,
                        0,
                        3,
                        ResourceAddress::sample(),
                        10,
                    )],
                )
                .unwrap(),
            r#"
            CALL_METHOD
                Address("accesscontroller_rdx1c0duj4lq0dc3cpl8qd420fpn5eckh8ljeysvjm894lyl5ja5yq6y5a")
                "create_proof"
            ;
            CALL_METHOD
                Address("account_rdx128dtethfy8ujrsfdztemyjk0kvhnah6dafr57frz85dcw2c8z0td87")
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
            ASSERT_WORKTOP_CONTAINS
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
}
