use super::{
    factor_source_id_from_hash::FactorSourceIDFromHash,
    hierarchical_deterministic_factor_instance::HierarchicalDeterministicFactorInstance,
};
use crate::{
    AccountPath, CAP26Path, HasEntityPath, HierarchicalDeterministicPublicKey, IdentityPath,
    IsEntityPath,
};
use crate::{CommonError as Error, PublicKey};
use derive_getters::Getters;

/// A specialized Hierarchical Deterministic FactorInstance used for transaction signing
/// and creation of virtual Accounts and Identities (Personas).
#[derive(Clone, Debug, PartialEq, Eq, Getters)]
pub struct HDFactorInstanceTransactionSigning<E: IsEntityPath> {
    factor_source_id: FactorSourceIDFromHash,

    #[getter(skip)]
    // We prefer to let the method `public_key()` return a `HierarchicalDeterministicPublicKey` instead of a `PublicKey` since it contains derivation path
    public_key: PublicKey,

    path: E,
}
impl<E: IsEntityPath + Clone> HDFactorInstanceTransactionSigning<E> {
    #[cfg(not(tarpaulin_include))] // false negative
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
                factor_source_id: hd_factor_instance.factor_source_id().clone(),
                public_key: hd_factor_instance.public_key().public_key().clone(),
                path: path.clone(),
            })
        } else {
            return Err(Error::WrongEntityKindOfInFactorInstancesPath);
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
        HierarchicalDeterministicPublicKey::new(
            self.public_key.clone(),
            self.path.derivation_path(),
        )
    }
}

/// Just an alias for when `HDFactorInstanceTransactionSigning` is used to create a new Account.
pub type HDFactorInstanceAccountCreation = HDFactorInstanceTransactionSigning<AccountPath>;

/// Just an alias for when `HDFactorInstanceTransactionSigning` is used to create a new Account.
pub type HDFactorInstanceIdentityCreation = HDFactorInstanceTransactionSigning<IdentityPath>;

impl HDFactorInstanceAccountCreation {
    pub fn new(hd_factor_instance: HierarchicalDeterministicFactorInstance) -> Result<Self, Error> {
        Self::try_from(hd_factor_instance, |p| p.as_account_path())
    }
}

impl<E: IsEntityPath + Clone> From<HDFactorInstanceTransactionSigning<E>>
    for HierarchicalDeterministicFactorInstance
{
    fn from(value: HDFactorInstanceTransactionSigning<E>) -> Self {
        HierarchicalDeterministicFactorInstance::new(
            value.clone().factor_source_id,
            value.public_key(),
        )
    }
}

impl HDFactorInstanceIdentityCreation {
    pub fn new(hd_factor_instance: HierarchicalDeterministicFactorInstance) -> Result<Self, Error> {
        Self::try_from(hd_factor_instance, |p| p.as_identity_path())
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        AccountPath, CAP26KeyKind, CAP26Repr, HierarchicalDeterministicPublicKey, IdentityPath,
        IsEntityPath,
    };
    use crate::{CommonError as Error, HasPlaceholder, PublicKey};

    use crate::{
        v100::factors::{
            factor_source_id_from_hash::FactorSourceIDFromHash,
            hd_transaction_signing_factor_instance::{
                HDFactorInstanceAccountCreation, HDFactorInstanceIdentityCreation,
            },
            hierarchical_deterministic_factor_instance::HierarchicalDeterministicFactorInstance,
        },
        NetworkID,
    };

    #[test]
    fn account_creation_valid() {
        let hd_key = HierarchicalDeterministicPublicKey::new(
            PublicKey::placeholder_ed25519(),
            AccountPath::placeholder().into(),
        );
        let hd_fi =
            HierarchicalDeterministicFactorInstance::new(
                FactorSourceIDFromHash::placeholder(),
                hd_key,
            );
        assert_eq!(
            HDFactorInstanceAccountCreation::new(hd_fi)
                .unwrap()
                .path()
                .key_kind(),
            CAP26KeyKind::TransactionSigning
        );
    }

    #[test]
    fn account_creation_wrong_entity_kind() {
        let hd_key = HierarchicalDeterministicPublicKey::new(
            PublicKey::placeholder_ed25519(),
            IdentityPath::placeholder().into(),
        );
        let hd_fi =
            HierarchicalDeterministicFactorInstance::new(
                FactorSourceIDFromHash::placeholder(),
                hd_key,
            );
        assert_eq!(
            HDFactorInstanceAccountCreation::new(hd_fi),
            Err(Error::WrongEntityKindOfInFactorInstancesPath)
        );
    }

    #[test]
    fn account_creation_wrong_key_kind() {
        let hd_key = HierarchicalDeterministicPublicKey::new(
            PublicKey::placeholder_ed25519(),
            AccountPath::new(
                NetworkID::Mainnet.into(),
                CAP26KeyKind::AuthenticationSigning,
                0,
            )
            .into(),
        );
        let hd_fi =
            HierarchicalDeterministicFactorInstance::new(
                FactorSourceIDFromHash::placeholder(),
                hd_key,
            );
        assert_eq!(
            HDFactorInstanceAccountCreation::new(hd_fi),
            Err(Error::WrongKeyKindOfTransactionSigningFactorInstance)
        );
    }

    #[test]
    fn identity_creation_valid() {
        let hd_key = HierarchicalDeterministicPublicKey::new(
            PublicKey::placeholder_ed25519(),
            IdentityPath::placeholder().into(),
        );
        let hd_fi =
            HierarchicalDeterministicFactorInstance::new(
                FactorSourceIDFromHash::placeholder(),
                hd_key,
            );
        assert_eq!(
            HDFactorInstanceIdentityCreation::new(hd_fi)
                .unwrap()
                .path()
                .key_kind(),
            CAP26KeyKind::TransactionSigning
        );
    }

    #[test]
    fn identity_creation_wrong_entity_kind() {
        let hd_key = HierarchicalDeterministicPublicKey::new(
            PublicKey::placeholder_ed25519(),
            AccountPath::placeholder().into(),
        );
        let hd_fi =
            HierarchicalDeterministicFactorInstance::new(
                FactorSourceIDFromHash::placeholder(),
                hd_key,
            );
        assert_eq!(
            HDFactorInstanceIdentityCreation::new(hd_fi),
            Err(Error::WrongEntityKindOfInFactorInstancesPath)
        );
    }

    #[test]
    fn identity_creation_wrong_key_kind() {
        let hd_key = HierarchicalDeterministicPublicKey::new(
            PublicKey::placeholder_ed25519(),
            IdentityPath::new(
                NetworkID::Mainnet.into(),
                CAP26KeyKind::AuthenticationSigning,
                0,
            )
            .into(),
        );
        let hd_fi =
            HierarchicalDeterministicFactorInstance::new(
                FactorSourceIDFromHash::placeholder(),
                hd_key,
            );
        assert_eq!(
            HDFactorInstanceIdentityCreation::new(hd_fi),
            Err(Error::WrongKeyKindOfTransactionSigningFactorInstance)
        );
    }
}
