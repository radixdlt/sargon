use std::sync::{Mutex, OnceLock};

use crate::prelude::*;
use addresses::address_union;

// TBD if this is needed, or just cleaned up.
address_union!(
    enum EntityApplyingShieldAddress: accessController, account, identity
);

impl From<AccountOrPersona> for EntityApplyingShieldAddress {
    fn from(value: AccountOrPersona) -> Self {
        match value.security_state() {
            EntitySecurityState::Securified { value } => {
                Self::AccessController(value.access_controller_address())
            }
            EntitySecurityState::Unsecured { .. } => match value {
                AccountOrPersona::AccountEntity(account) => {
                    Self::Account(account.address)
                }
                AccountOrPersona::PersonaEntity(persona) => {
                    Self::Identity(persona.address)
                }
            },
        }
    }
}

fn hacky_tmp_entities_applying_shield(
) -> &'static Mutex<IndexMap<EntityApplyingShieldAddress, TransactionManifest>>
{
    static ARRAY: OnceLock<
        Mutex<IndexMap<EntityApplyingShieldAddress, TransactionManifest>>,
    > = OnceLock::new();
    ARRAY.get_or_init(|| Mutex::new(IndexMap::new()))
}

pub fn hacky_tmp_get_entities_applying_shield(
) -> IndexMap<EntityApplyingShieldAddress, TransactionManifest> {
    hacky_tmp_entities_applying_shield().lock().unwrap().clone()
}

// TODO: when https://github.com/radixdlt/sargon/pull/373 and follow up PRs are merged and we can get those addresses from the manifest using RETs analysis
// is merge remove this and use static analysis using RET to get this.
fn __hacky_tmp_using_local_global_state_extract_address_of_entity_updating_shield(
    manifest: &TransactionManifest,
) -> Result<EntityApplyingShieldAddress> {
    let lookup = hacky_tmp_get_entities_applying_shield();
    let address = lookup.iter().find_map(|(address, m)| {
        if m == manifest {
            Some(*address)
        } else {
            None
        }
    });
    address.ok_or(CommonError::Unknown {
        error_message: "Failed extracting address of entity updating shield"
            .to_string(),
    })
}

// TODO: when https://github.com/radixdlt/sargon/pull/373 and follow up PRs are merged
// impl this
fn _extract_address_of_entity_updating_shield(
    _manifest: &TransactionManifest,
) -> Result<EntityApplyingShieldAddress> {
    todo!("cannot be implemented yet, awaiting #132 RET PR")
}

// TODO: when https://github.com/radixdlt/sargon/pull/373 and follow up PRs are merged
// is merge remove this and use static analysis using RET to get this.
pub fn extract_address_of_entity_updating_shield(
    manifest: &TransactionManifest,
) -> Result<EntityApplyingShieldAddress> {
    __hacky_tmp_using_local_global_state_extract_address_of_entity_updating_shield(manifest)
}
