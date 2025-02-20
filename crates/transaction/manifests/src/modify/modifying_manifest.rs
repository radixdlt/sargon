use crate::prelude::*;
use radix_engine_interface::blueprints::access_controller::ACCESS_CONTROLLER_CREATE_PROOF_IDENT;

pub trait ModifyingManifest<M, I>
where
    M: IntoManifest<I> + Clone,
    I: IntoInstruction + Clone,
    Self: Sized,
{
    fn modifying_manifest(&self) -> M;
}

pub struct InstructionPosition(pub u64);

pub trait AddingGuaranteesModifyingManifest<M, I>:
    ModifyingManifest<M, I>
where
    M: IntoManifest<I> + Clone,
    I: IntoInstruction + Clone,
{
    fn insert_guarantee_assertion_at_position(
        self,
        position: InstructionPosition,
        guarantee: TransactionGuarantee,
    ) -> Result<Self>;

    // Modifies `manifest` by inserting transaction "guarantees", which is the wallet
    // term for `assert_worktop_contains`.
    fn modify_add_guarantees<G>(self, guarantees: G) -> Result<M>
    where
        G: IntoIterator<Item = TransactionGuarantee>,
    {
        let guarantees = guarantees.into_iter().collect_vec();
        if guarantees.is_empty() {
            return Ok(self.modifying_manifest());
        };

        let instructions = self.modifying_manifest().instructions();
        let instruction_count = instructions.len() as u64;

        if let Some(oob) = guarantees
            .clone()
            .into_iter()
            .find(|g| g.instruction_index >= instruction_count)
        {
            return Err(CommonError::TXGuaranteeIndexOutOfBounds {
                index: oob.instruction_index,
                count: instruction_count,
            });
        }

        // Will be increased with each added guarantee to account for the
        // difference in indexes from the initial manifest.
        let mut offset = 0;
        let mut modifiable_manifest = self;

        for guarantee in guarantees {
            modifiable_manifest = modifiable_manifest
                .insert_guarantee_assertion_at_position(
                    InstructionPosition(guarantee.instruction_index + offset),
                    guarantee,
                )?;

            offset.add_assign(1);
        }

        Ok(modifiable_manifest.modifying_manifest())
    }
}

/// Used by development, in production we SHOULD use the fee given by analyzing
/// the manifest.
fn default_fee() -> Decimal192 {
    Decimal192::from(25)
}

pub struct LockFeeData {
    pub fee_payer_address: AccountAddress,
    fee: Option<Decimal192>,
}

impl LockFeeData {
    pub fn new_with_fee(
        fee_payer_address: AccountAddress,
        fee: Decimal192,
    ) -> Self {
        Self {
            fee_payer_address,
            fee: Some(fee),
        }
    }

    pub fn new_with_fee_payer(fee_payer_address: AccountAddress) -> Self {
        Self {
            fee_payer_address,
            fee: None,
        }
    }

    pub fn fee(&self) -> Decimal192 {
        self.fee.unwrap_or(default_fee())
    }
}

pub trait AddingLockFeeAndProofsModifyingManifest<M, B, I>:
    ModifyingManifest<M, I>
where
    M: IntoManifest<I> + Clone,
    I: IntoInstruction + Clone,
    B: IntoManifestBuilder<M, I>,
{
    fn modify_add_proofs(
        &self,
        entities_with_access_controllers: IndexMap<
            AddressOfAccountOrPersona,
            AccessControllerAddress,
        >,
    ) -> Result<M> {
        self.modify_add_lock_fee_and_proofs(
            None,
            entities_with_access_controllers,
        )
    }

    fn modify_add_lock_fee_and_proofs(
        &self,
        lock_fee_data: impl Into<Option<LockFeeData>>,
        entities_with_access_controllers: IndexMap<
            AddressOfAccountOrPersona,
            AccessControllerAddress,
        >,
    ) -> Result<M> {
        let mut access_controllers = entities_with_access_controllers
            .iter()
            .map(|(_, ac)| *ac)
            .collect::<IndexSet<_>>();

        let mut builder = B::new_with_instructions([]);

        if let Some(lock_fee_data) = lock_fee_data.into() {
            let lock_fee_entity_address = AddressOfAccountOrPersona::Account(
                lock_fee_data.fee_payer_address,
            );

            if let Some(access_controller_of_fee_payer) =
                entities_with_access_controllers.get(&lock_fee_entity_address)
            {
                access_controllers.shift_remove(access_controller_of_fee_payer);

                // Add proof for lock fee payer, who happens to be securified.
                builder = builder.call_method(
                    ScryptoGlobalAddress::from(*access_controller_of_fee_payer),
                    ACCESS_CONTROLLER_CREATE_PROOF_IDENT,
                    (),
                );
            }

            // Add lock fee
            builder = builder
                .lock_fee(&lock_fee_data.fee_payer_address, lock_fee_data.fee())
        }

        // Put the remaining proofs of the Access Controller addresses
        for access_controller in access_controllers {
            builder = builder.call_method(
                ScryptoGlobalAddress::from(access_controller),
                ACCESS_CONTROLLER_CREATE_PROOF_IDENT,
                (),
            );
        }

        let modifying_manifest = self.modifying_manifest();

        builder =
            builder.extend_builder_with_manifest(modifying_manifest.clone());
        builder.build(modifying_manifest.network_id())
    }
}
