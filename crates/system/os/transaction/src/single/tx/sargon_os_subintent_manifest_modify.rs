use crate::prelude::*;

pub trait SargonOsSubintentManifestModify {
    fn modify_subintent_manifest<G>(
        &self,
        manifest: SubintentManifest,
        guarantees: G,
    ) -> Result<SubintentManifest>
    where
        G: IntoIterator<Item = TransactionGuarantee>;
}

impl SargonOsSubintentManifestModify for SargonOS {
    fn modify_subintent_manifest<G>(
        &self,
        manifest: SubintentManifest,
        guarantees: G,
    ) -> Result<SubintentManifest>
    where
        G: IntoIterator<Item = TransactionGuarantee>,
    {
        let summary = manifest.summary()?;
        let proofs_for_entities_requiring_auth =
            self.extract_proofs(&summary)?;

        // Add the potential `create_proof`s
        let modified_manifest =
            manifest.modify_add_proofs(proofs_for_entities_requiring_auth)?;

        // Lastly add the guarantees
        modified_manifest.modify_add_guarantees(guarantees)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::single::support::{
        prepare_os_with_entities, prepare_xrd_transfer_transaction,
    };

    #[actix_rt::test]
    async fn test_modify_subintent_manifest_adds_guarantees_in_same_index() {
        let acc1 = Account::sample_at(3);
        let acc2 = Account::sample_at(4);

        let os =
            prepare_os_with_entities([acc1.clone(), acc2.clone()], []).await;

        let subintent = prepare_sunbintent_xrd_transfer_transaction(
            acc1.address(),
            acc2.address(),
        );

        subintent_manifest_eq(
            os.modify_subintent_manifest(subintent, [guarantee(4)])
                .unwrap(),
            r#"
            CALL_METHOD
                Address("accesscontroller_rdx1cdgcthvtzcny04t5dnuc0wq5n8hx90eytn6luhmfm5g52rm6wvsnfk")
                "create_proof"
            ;
            CALL_METHOD
                Address("accesscontroller_rdx1cdgewxe4wmg69544jrh956srstgvm05z7yzfnasxmnfd938uywgmgd")
                "create_proof"
            ;
            CALL_METHOD
                Address("account_rdx12xzamzckyera2arvlxrms9yeae3t7fzu7hl976waz9zs7752sljdkf")
                "lock_fee"
                Decimal("500")
            ;
            CALL_METHOD
                Address("account_rdx12xzamzckyera2arvlxrms9yeae3t7fzu7hl976waz9zs7752sljdkf")
                "withdraw"
                Address("resource_rdx1tknxxxxxxxxxradxrdxxxxxxxxx009923554798xxxxxxxxxradxrd")
                Decimal("5")
            ;
            ASSERT_WORKTOP_CONTAINS
                Address("resource_rdx1tknxxxxxxxxxradxrdxxxxxxxxx009923554798xxxxxxxxxradxrd")
                Decimal("1339")
            ;
            TAKE_FROM_WORKTOP
                Address("resource_rdx1tknxxxxxxxxxradxrdxxxxxxxxx009923554798xxxxxxxxxradxrd")
                Decimal("5")
                Bucket("bucket1")
            ;
            CALL_METHOD
                Address("account_rdx12xt3kdtk6x3d9dvsaedx5quz6rxmaqh3qjvlvpku6tfvflpj0nz4qq")
                "deposit"
                Bucket("bucket1")
            ;
            YIELD_TO_PARENT;
            "#,
        )
    }

    fn guarantee(index: u64) -> TransactionGuarantee {
        TransactionGuarantee::new(
            1339,
            0,
            index,
            ResourceAddress::xrd_on_network(NetworkID::Mainnet),
            18,
        )
    }

    fn prepare_sunbintent_xrd_transfer_transaction(
        from: AccountAddress,
        to: AccountAddress,
    ) -> SubintentManifest {
        let manifest = prepare_xrd_transfer_transaction(from, to);

        let mut instructions_str = manifest.instructions.instructions_string();
        instructions_str.insert_str(
            0,
        r#"
CALL_METHOD
    Address("account_rdx12xzamzckyera2arvlxrms9yeae3t7fzu7hl976waz9zs7752sljdkf")
    "lock_fee"
    Decimal("500")
;
        "#);
        instructions_str.push_str(
            r#"
YIELD_TO_PARENT;
            "#,
        );

        let instructions =
            InstructionsV2::new(instructions_str, NetworkID::Mainnet).unwrap();

        SubintentManifest::with_instructions_and_blobs_and_children(
            instructions,
            Blobs::default(),
            ChildSubintentSpecifiers::default(),
        )
    }
}
