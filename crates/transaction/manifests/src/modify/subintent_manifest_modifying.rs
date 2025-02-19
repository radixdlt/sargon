use crate::prelude::*;

pub trait SubintentManifestModifying {
    /// Modifies the subintent manifest applying the following instructions
    /// - attaches `AccessControllerAddress` proofs for `entities_with_access_controllers`,
    /// - adds guarantee assertions in specific indices described in `guarantees`. Remember that
    ///   those indices are received from `transaction/preview` were `lock_fee` is not present.
    fn modify<G>(
        self,
        entities_with_access_controllers: IndexMap<
            AddressOfAccountOrPersona,
            AccessControllerAddress,
        >,
        guarantees: G,
    ) -> Result<SubintentManifest>
    where
        G: IntoIterator<Item = TransactionGuarantee>;
}

impl SubintentManifestModifying for SubintentManifest {
    fn modify<G>(
        self,
        entities_with_access_controllers: IndexMap<
            AddressOfAccountOrPersona,
            AccessControllerAddress,
        >,
        guarantees: G,
    ) -> Result<SubintentManifest>
    where
        G: IntoIterator<Item = TransactionGuarantee>,
    {
        let modified = self.modify_add_proofs_and_lock_fee(
            None,
            entities_with_access_controllers,
        )?;

        modified.modify_add_guarantees(guarantees)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_modify() {
        let manifest = SubintentManifest::sample();

        let dapp_fee_payer_address = AccountAddress::try_from_bech32(
            "account_rdx128y6j78mt0aqv6372evz28hrxp8mn06ccddkr7xppc88hyvynvjdwr"
        ).unwrap();

        let securified_entities = IndexMap::from([
            (
                AddressOfAccountOrPersona::from(dapp_fee_payer_address),
                AccessControllerAddress::sample_mainnet(),
            ),
            (
                AddressOfAccountOrPersona::sample_mainnet_other(),
                AccessControllerAddress::sample_mainnet_other(),
            ),
        ]);

        subintent_manifest_eq(
            manifest
                .modify(
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
                Address("accesscontroller_rdx1cv93xuha64eay8ctkx9km0el2jgkuh6gqlwec7tzecccyu0rj37xak")
                "create_proof"
            ;
            CALL_METHOD
                Address("account_rdx128y6j78mt0aqv6372evz28hrxp8mn06ccddkr7xppc88hyvynvjdwr")
                "lock_fee"
                Decimal("0.61")
            ;
            CALL_METHOD
                Address("account_rdx128y6j78mt0aqv6372evz28hrxp8mn06ccddkr7xppc88hyvynvjdwr")
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
                Address("account_rdx12xkzynhzgtpnnd02tudw2els2g9xl73yk54ppw8xekt2sdrlaer264")
                "try_deposit_or_abort"
                Bucket("bucket1")
                Enum<0u8>()
            ;
            YIELD_TO_PARENT;
            "#,
        );
    }
}
