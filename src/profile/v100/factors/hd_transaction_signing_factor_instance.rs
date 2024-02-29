use crate::prelude::*;

/// A specialized Hierarchical Deterministic FactorInstance used for transaction signing
/// and creation of virtual Accounts and Identities (Personas).
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct HDFactorInstanceTransactionSigning<E: IsEntityPath + Clone> {
    pub factor_source_id: FactorSourceIDFromHash,
    pub public_key: PublicKey,
    pub path: E,
}

impl<T: IsEntityPath + Clone> HDFactorInstanceTransactionSigning<T> {
    fn try_from_factor_instance(
        value: HierarchicalDeterministicFactorInstance,
    ) -> Result<Self> {
        value
            .derivation_path()
            .as_cap26()
            .ok_or(CommonError::WrongEntityKindOfInFactorInstancesPath)
            .map(|p| p.clone())
            .and_then(|p| {
                p.try_into()
                    .map_err(|_| CommonError::WrongEntityKindOfInFactorInstancesPath)
            })
            .and_then(|p: T| {
                if !p.key_kind().is_transaction_signing() {
                    Err(CommonError::WrongKeyKindOfTransactionSigningFactorInstance)
                } else {
                    Ok(p)
                }
            })
            .map(|p| Self {
                factor_source_id: value.factor_source_id.clone(),
                public_key: value.public_key.public_key.clone(),
                path: p.clone(),
            })
    }
}

impl<E: IsEntityPath + Clone> HasEntityPath<E>
    for HDFactorInstanceTransactionSigning<E>
{
    fn path(&self) -> E {
        self.path.clone()
    }
}

impl<E: IsEntityPath + Clone> HDFactorInstanceTransactionSigning<E> {
    pub fn public_key(&self) -> HierarchicalDeterministicPublicKey {
        HierarchicalDeterministicPublicKey::new(
            self.public_key.clone(),
            self.path.derivation_path(),
        )
    }
}

/// Just an alias for when `HDFactorInstanceTransactionSigning` is used to create a new Account.
pub type HDFactorInstanceAccountCreation =
    HDFactorInstanceTransactionSigning<AccountPath>;

/// Just an alias for when `HDFactorInstanceTransactionSigning` is used to create a new Account.
pub type HDFactorInstanceIdentityCreation =
    HDFactorInstanceTransactionSigning<IdentityPath>;

impl<T> HDFactorInstanceTransactionSigning<T>
where
    T: IsEntityPath + Clone,
{
    pub fn new(
        hd_factor_instance: HierarchicalDeterministicFactorInstance,
    ) -> Result<Self> {
        Self::try_from_factor_instance(hd_factor_instance)
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

#[cfg(test)]
mod tests {
    use crate::prelude::*;

    #[test]
    fn account_creation_valid() {
        let hd_key = HierarchicalDeterministicPublicKey::new(
            PublicKey::sample_ed25519(),
            AccountPath::sample().into(),
        );
        let hd_fi = HierarchicalDeterministicFactorInstance::new(
            FactorSourceIDFromHash::sample(),
            hd_key,
        );
        assert_eq!(
            HDFactorInstanceAccountCreation::new(hd_fi)
                .unwrap()
                .path
                .key_kind(),
            CAP26KeyKind::TransactionSigning
        );
    }

    #[test]
    fn account_creation_wrong_entity_kind() {
        let hd_key = HierarchicalDeterministicPublicKey::new(
            PublicKey::sample_ed25519(),
            IdentityPath::sample().into(),
        );
        let hd_fi = HierarchicalDeterministicFactorInstance::new(
            FactorSourceIDFromHash::sample(),
            hd_key,
        );
        assert_eq!(
            HDFactorInstanceAccountCreation::new(hd_fi),
            Err(CommonError::WrongEntityKindOfInFactorInstancesPath)
        );
    }

    #[test]
    fn account_creation_wrong_key_kind() {
        let hd_key = HierarchicalDeterministicPublicKey::new(
            PublicKey::sample_ed25519(),
            AccountPath::new(
                NetworkID::Mainnet,
                CAP26KeyKind::AuthenticationSigning,
                0,
            )
            .into(),
        );
        let hd_fi = HierarchicalDeterministicFactorInstance::new(
            FactorSourceIDFromHash::sample(),
            hd_key,
        );
        assert_eq!(
            HDFactorInstanceAccountCreation::new(hd_fi),
            Err(CommonError::WrongKeyKindOfTransactionSigningFactorInstance)
        );
    }

    #[test]
    fn identity_creation_valid() {
        let hd_key = HierarchicalDeterministicPublicKey::new(
            PublicKey::sample_ed25519(),
            IdentityPath::sample().into(),
        );
        let hd_fi = HierarchicalDeterministicFactorInstance::new(
            FactorSourceIDFromHash::sample(),
            hd_key,
        );
        assert_eq!(
            HDFactorInstanceIdentityCreation::new(hd_fi)
                .unwrap()
                .path
                .key_kind(),
            CAP26KeyKind::TransactionSigning
        );
    }

    #[test]
    fn identity_creation_wrong_entity_kind() {
        let hd_key = HierarchicalDeterministicPublicKey::new(
            PublicKey::sample_ed25519(),
            AccountPath::sample().into(),
        );
        let hd_fi = HierarchicalDeterministicFactorInstance::new(
            FactorSourceIDFromHash::sample(),
            hd_key,
        );
        assert_eq!(
            HDFactorInstanceIdentityCreation::new(hd_fi),
            Err(CommonError::WrongEntityKindOfInFactorInstancesPath)
        );
    }

    #[test]
    fn identity_creation_wrong_key_kind() {
        let hd_key = HierarchicalDeterministicPublicKey::new(
            PublicKey::sample_ed25519(),
            IdentityPath::new(
                NetworkID::Mainnet,
                CAP26KeyKind::AuthenticationSigning,
                0,
            )
            .into(),
        );
        let hd_fi = HierarchicalDeterministicFactorInstance::new(
            FactorSourceIDFromHash::sample(),
            hd_key,
        );
        assert_eq!(
            HDFactorInstanceIdentityCreation::new(hd_fi),
            Err(CommonError::WrongKeyKindOfTransactionSigningFactorInstance)
        );
    }
}
