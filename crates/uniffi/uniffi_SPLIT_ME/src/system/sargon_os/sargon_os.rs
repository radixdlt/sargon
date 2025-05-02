use crate::prelude::*;
use sargon::Interactors;
use sargon::SargonOS as InternalSargonOS;

/// The Sargon "Operating System" is the root "manager" of the Sargon library
/// which holds an in-memory Profile and a collection of "clients" which are
/// created from "drivers" which the hosts (iOS/Android wallets) "installs"
/// during app launch, enabling the  Sargon "Operating System" to e.g read/write
/// to secure storage and make use of the network connection of the iPhone/Android
/// phone.
#[derive(uniffi::Object)]
pub struct SargonOS {
    pub(crate) wrapped: Arc<InternalSargonOS>,
}

#[uniffi::export]
impl SargonOS {
    #[uniffi::constructor]
    pub async fn boot(
        bios: Arc<Bios>,
        interactor: Arc<dyn HostInteractor>,
    ) -> Result<Arc<Self>> {
        let internal_sargon_os = InternalSargonOS::boot(
            Arc::new(bios.as_ref().clone().into()),
            Interactors::new(
                Arc::new(UseFactorSourcesInteractorAdapter::new(
                    interactor.clone(),
                )),
                Arc::new(AuthorizationInteractorAdapter::new(
                    interactor.clone(),
                )),
                Arc::new(SpotCheckInteractorAdapter::new(interactor)),
            ),
        )
        .await;

        Result::Ok(Arc::new(SargonOS {
            wrapped: internal_sargon_os,
        }))
    }

    pub async fn new_wallet(
        &self,
        should_pre_derive_instances: bool,
    ) -> Result<()> {
        self.wrapped
            .new_wallet(should_pre_derive_instances)
            .await
            .into_result()
    }

    pub async fn import_wallet(
        &self,
        profile: &Profile,
        bdfs_skipped: bool,
    ) -> Result<()> {
        self.wrapped
            .import_wallet(&profile.into_internal(), bdfs_skipped)
            .await
            .into_result()
    }

    pub async fn new_wallet_with_derived_bdfs(
        &self,
        hd_factor_source: PrivateHierarchicalDeterministicFactorSource,
        accounts: Vec<Account>,
    ) -> Result<()> {
        self.wrapped
            .new_wallet_with_derived_bdfs(
                hd_factor_source.into_internal(),
                accounts.into_internal(),
            )
            .await
            .into_result()
    }

    pub async fn delete_wallet(&self) -> Result<()> {
        self.wrapped.delete_wallet().await.into_result()
    }

    pub fn host_id(&self) -> HostId {
        self.wrapped.host_id().into()
    }

    pub async fn resolve_host_info(&self) -> HostInfo {
        self.wrapped.resolve_host_info().await.into()
    }

    pub async fn claim_profile(&self, profile: Profile) -> Profile {
        let mut internal_profile = profile.into_internal();
        self.wrapped.claim_profile(&mut internal_profile).await;
        internal_profile.into()
    }
}
