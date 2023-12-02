use super::{
    factor_source_id_from_hash::FactorSourceIDFromHash,
    hierarchical_deterministic_factor_instance::HierarchicalDeterministicFactorInstance,
};
use hierarchical_deterministic::{
    cap26::cap26_path::{
        cap26_path::CAP26Path,
        paths::{
            account_path::AccountPath,
            is_entity_path::{HasEntityPath, IsEntityPath},
        },
    },
    derivation::hierarchical_deterministic_public_key::HierarchicalDeterministicPublicKey,
};
use wallet_kit_common::{
    error::common_error::CommonError as Error, types::keys::public_key::PublicKey,
};

/// A specialized Hierarchical Deterministic FactorInstance used for transaction signing
/// and creation of virtual Accounts and Identities (Personas).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct HDFactorInstanceTransactionSigning<E: IsEntityPath> {
    pub factor_source_id: FactorSourceIDFromHash,
    public_key: PublicKey,
    pub path: E,
}
impl<E: IsEntityPath + Clone> HDFactorInstanceTransactionSigning<E> {
    pub fn try_from<F>(
        hd_factor_instance: HierarchicalDeterministicFactorInstance,
        extract: F,
    ) -> Result<Self, Error>
    where
        F: Fn(&CAP26Path) -> Option<&E>,
    {
        if let Some(path) = hd_factor_instance
            .derivation_path()
            .as_cap26()
            .and_then(|p| extract(p))
        {
            if !path.key_kind().is_transaction_signing() {
                return Err(Error::WrongKeyKindOfTransactionSigningFactorInstance);
            }

            Ok(Self {
                factor_source_id: hd_factor_instance.factor_source_id,
                public_key: hd_factor_instance.public_key.public_key,
                path: path.clone(),
            })
        } else {
            return Err(Error::WrongKeyKindOfTransactionSigningFactorInstance);
        }
    }
}

impl<E: IsEntityPath + Clone> HasEntityPath<E> for HDFactorInstanceTransactionSigning<E> {
    fn path(&self) -> E {
        self.path.clone()
    }
}

impl<E: IsEntityPath> HDFactorInstanceTransactionSigning<E> {
    pub fn public_key(&self) -> HierarchicalDeterministicPublicKey {
        HierarchicalDeterministicPublicKey::new(self.public_key, self.path.derivation_path())
    }
}

/// Just an alias for when `HDFactorInstanceTransactionSigning` is used in the purpose of
/// creation of a virtual entity - i.e. derivation of entity address.
pub type HDFactorInstanceAccountCreation = HDFactorInstanceTransactionSigning<AccountPath>;

impl HDFactorInstanceAccountCreation {
    pub fn new(hd_factor_instance: HierarchicalDeterministicFactorInstance) -> Result<Self, Error> {
        Self::try_from(hd_factor_instance, |p| p.as_account_path())
    }
}

impl From<HDFactorInstanceAccountCreation> for HierarchicalDeterministicFactorInstance {
    fn from(value: HDFactorInstanceAccountCreation) -> Self {
        HierarchicalDeterministicFactorInstance::new(
            value.clone().factor_source_id,
            value.public_key(),
        )
    }
}
