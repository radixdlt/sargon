use crate::prelude::*;
use std::future::Future;

impl Profile {
    /// Creates a new non securified account **WITHOUT** adding it to Profile, using the *main* "Babylon"
    /// `DeviceFactorSource` and the "next" index for this FactorSource as derivation path.
    ///
    /// If you want to add it to Profile, call `add_account(account)`
    ///
    /// Returns a tuple `(FactorSourceID, Account)` where FactorSourceID is the ID
    /// of the FactorSource used to create the account.
    pub async fn create_unsaved_account(
        &self,
        network_id: NetworkID,
        name: DisplayName,
        factor_instances_cache_client: &FactorInstancesCacheClient,
        key_derivation_interactors: Arc<dyn KeysDerivationInteractors>,
    ) -> Result<(FactorSourceID, Account)> {
        let (factor_source_id, accounts) = self
            .create_unsaved_accounts(
                network_id,
                1,
                factor_instances_cache_client,
                key_derivation_interactors,
                |_| name.clone(),
            )
            .await?;

        let account = accounts
            .into_iter()
            .last()
            .expect("Should have created one account");

        Ok((factor_source_id, account))
    }

    /// Creates many new non securified accounts **WITHOUT** adding them to Profile, using the *main* "Babylon"
    /// `DeviceFactorSource` and the "next" indices for this FactorSource as derivation paths.
    ///
    /// If you want to add the accounts to Profile, call `add_accounts(accounts)`
    ///
    /// Returns a tuple `(FactorSourceID, Accounts)` where FactorSourceID is the ID
    /// of the FactorSource used to create the accounts.
    pub async fn create_unsaved_accounts(
        &self,
        network_id: NetworkID,
        count: u16,
        factor_instances_cache_client: &FactorInstancesCacheClient,
        key_derivation_interactors: Arc<dyn KeysDerivationInteractors>,
        get_name: impl Fn(u32) -> DisplayName, // name of account at index
    ) -> Result<(FactorSourceID, Accounts)> {
        let count = count as usize;

        let number_of_accounts_on_network = self
            .networks
            .get_id(network_id)
            .map(|n| n.accounts.len())
            .unwrap_or(0);

        let bdfs = self.bdfs();
        let fsid = bdfs.factor_source_id();

        let outcome =
            VirtualEntityCreatingInstanceProvider::for_many_account_vecis(
                count,
                factor_instances_cache_client,
                Some(self),
                bdfs.into(),
                network_id,
                key_derivation_interactors,
            )
            .await?;

        let instances_to_use_directly = outcome.to_use_directly;

        assert_eq!(instances_to_use_directly.len(), count);

        let accounts = instances_to_use_directly
            .into_iter()
            .map(|f| {
                HDFactorInstanceTransactionSigning::<AccountPath>::new(f)
                    .unwrap()
            })
            .map(|veci| {
                let idx =
                    u32::from(veci.path.index().index_in_local_key_space());
                let name = get_name(idx);
                let appearance_id =
                    AppearanceID::from_number_of_accounts_on_network(
                        (idx as usize) + number_of_accounts_on_network,
                    );

                Account::new(veci, name, appearance_id)
            })
            .collect::<Accounts>();

        Ok((fsid, accounts))
    }
}

#[derive(Debug)]
pub struct NoUIDerivationInteractors {
    pub(crate) poly: Arc<dyn PolyFactorKeyDerivationInteractor>,
    pub(crate) mono: Arc<dyn MonoFactorKeyDerivationInteractor>,
}

#[derive(Debug)]
pub struct NoUIDerivationInteractorsPolyAndMono {
    lookup_mnemonic:
        fn(FactorSourceIDFromHash) -> Result<MnemonicWithPassphrase>,
}
impl NoUIDerivationInteractorsPolyAndMono {
    fn derive_mono(
        &self,
        request: MonoFactorKeyDerivationRequest,
    ) -> Result<IndexSet<HierarchicalDeterministicFactorInstance>> {
        __do_derive_serially_with_lookup_of_mnemonic(
            request,
            self.lookup_mnemonic,
        )
    }
}

#[async_trait::async_trait]
impl MonoFactorKeyDerivationInteractor
    for NoUIDerivationInteractorsPolyAndMono
{
    async fn derive(
        &self,
        request: MonoFactorKeyDerivationRequest,
    ) -> Result<KeyDerivationResponse> {
        let instances = self.derive_mono(request.clone())?;
        Ok(KeyDerivationResponse::new(IndexMap::just((
            request.factor_source_id,
            instances,
        ))))
    }
}

#[async_trait::async_trait]
impl PolyFactorKeyDerivationInteractor
    for NoUIDerivationInteractorsPolyAndMono
{
    async fn derive(
        &self,
        request: PolyFactorKeyDerivationRequest,
    ) -> Result<KeyDerivationResponse> {
        let pairs_result: Result<
            IndexMap<
                FactorSourceIDFromHash,
                IndexSet<HierarchicalDeterministicFactorInstance>,
            >,
        > = request
            .per_factor_source
            .into_iter()
            .map(|(k, r)| {
                let instances = self.derive_mono(r);
                instances.map(|i| (k, i))
            })
            .collect();
        let pairs = pairs_result?;
        Ok(KeyDerivationResponse::new(pairs))
    }
}

impl NoUIDerivationInteractors {
    fn with(
        poly: impl PolyFactorKeyDerivationInteractor + 'static,
        mono: impl MonoFactorKeyDerivationInteractor + 'static,
    ) -> Self {
        Self {
            poly: Arc::new(poly),
            mono: Arc::new(mono),
        }
    }
    pub fn new(secure_storage: &SecureStorageClient) -> Self {
        /*
           self.secure_storage.load_mnemonic_with_passphrase(id).await
        */
        todo!()
    }
}
impl KeysDerivationInteractors for NoUIDerivationInteractors {
    fn interactor_for(
        &self,
        kind: FactorSourceKind,
    ) -> KeyDerivationInteractor {
        match kind {
            FactorSourceKind::Device => {
                KeyDerivationInteractor::poly(self.poly.clone())
            }
            _ => KeyDerivationInteractor::mono(self.mono.clone()),
        }
    }
}

/*
#[cfg(test)]
mod tests {
    use super::*;

    #[actix_rt::test]
    async fn test_create_unsaved_accounts() {
        let sut = Profile::from_device_factor_source(
            PrivateHierarchicalDeterministicFactorSource::sample()
                .factor_source,
            HostId::sample(),
            HostInfo::sample(),
            None::<Accounts>,
        );

        let (_, accounts) = sut
            .create_unsaved_accounts(
                NetworkID::Mainnet,
                3,
                |i| {
                    DisplayName::new(if i == 0 {
                        "Alice"
                    } else if i == 1 {
                        "Bob"
                    } else {
                        "Carol"
                    })
                    .unwrap()
                },
                async move |_| {
                    Ok(PrivateHierarchicalDeterministicFactorSource::sample())
                },
            )
            .await
            .unwrap();

        pretty_assertions::assert_eq!(
            accounts,
            Accounts::from_iter([
                Account::sample_mainnet_alice(),
                Account::sample_mainnet_bob(),
                Account::sample_mainnet_carol()
            ])
        )
    }
}

*/
