use crate::prelude::*;

pub trait SargonOsTransactionModify {
    fn modify_transaction_manifest<G>(
        &self,
        manifest: TransactionManifest,
        fee_payer_address: AccountAddress,
        fee: Decimal192,
        guarantees: G,
    ) -> Result<TransactionManifest>
    where
        G: IntoIterator<Item = TransactionGuarantee>;
}

impl SargonOsTransactionModify for SargonOS {
    fn modify_transaction_manifest<G>(
        &self,
        manifest: TransactionManifest,
        fee_payer_address: AccountAddress,
        fee: Decimal192,
        guarantees: G,
    ) -> Result<TransactionManifest>
    where
        G: IntoIterator<Item = TransactionGuarantee>,
    {
        let summary = manifest.summary()?;
        let proofs = self.extract_proofs(&summary)?;

        // Resolve lock fee data
        let account_fee_payer = self.account_by_address(fee_payer_address)?;
        let lock_fee_data = if let Some(control) =
            account_fee_payer.security_state().as_securified()
        {
            // Fee payer is securified, access controller proof needs to be prepended
            // before lock fee.
            LockFeeData::new_with_securified_fee_payer(
                fee_payer_address,
                control.access_controller_address,
                fee,
            )
        } else {
            // Fee payer is unsecure, just lock fee instruction is added.
            LockFeeData::new_with_unsecurified_fee_payer(fee_payer_address, fee)
        };

        let fee_payer_entity_address =
            AddressOfAccountOrPersona::from(lock_fee_data.fee_payer_address);
        let offset = if proofs.contains_key(&fee_payer_entity_address) {
            // The difference of the summarised manifest and the modified one is just
            // one `lock_fee` instruction. If the fee payer is in the original set of entities
            // requiring proof then the `create_proof` instruction is already taken into account.
            1
        } else {
            // The difference of the summarised manifest and the modified one is the
            // `create_proof` & `lock_fee` instructions. Since the fee payer is not included
            // in the original securified set of entities requiring auth, then these 2 instructions
            // are added to the modified manifest.
            2
        };

        // Then add the `lock_fee` instruction with potential `create_proof`s
        let modified_manifest =
            manifest.modify_add_lock_fee_and_proofs(lock_fee_data, proofs)?;

        // Lastly add the guarantees with indices offset by `offset`
        modified_manifest.modify_add_guarantees(
            guarantees
                .into_iter()
                .map(|g| g.offset_instruction_index_by(offset)),
        )
    }
}
