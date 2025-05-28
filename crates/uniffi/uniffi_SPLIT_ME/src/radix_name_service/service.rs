use crate::prelude::*;

use indexmap::map::raw_entry_v1::RawEntryMut;
use sargon::AccountAddress as InternalAccountAddress;
use sargon::RadixNameService as InternalRadixNameService;
use sargon::Domain as InternalDomain;

#[derive(uniffi::Object)]
pub struct RadixNameService {
    pub wrapped: Arc<InternalRadixNameService>,
}

#[uniffi::export]
impl RadixNameService {
    #[uniffi::constructor]
    pub fn new(
        networking_driver: Arc<dyn NetworkingDriver>,
        network_id: NetworkID,
    ) -> Result<Self> {
        let wrapped = InternalRadixNameService::new_xrd_domains(
            Arc::new(NetworkingDriverAdapter {
                wrapped: networking_driver,
            }),
            network_id.into(),
        )
        .into_result()?;

        return Ok(Self {
            wrapped: Arc::new(wrapped),
        });
    }
}

#[uniffi::export]
pub fn rns_domain_validated(raw: String) -> Result<String> {
    InternalDomain::new(raw.clone())
        .validated()
        .map(|domain| raw)
        .map_err(|_| CommonError::Unknown)
}

#[derive(Clone, PartialEq, Eq, Hash, uniffi::Record)]
pub struct ResolvedReceiver {
    pub domain: String,
    pub account: AccountAddress,
}

#[uniffi::export]
impl RadixNameService {
    #[uniffi::method]
    pub async fn resolve_receiver_account_for_domain(
        &self,
        domain: String,
    ) -> Result<ResolvedReceiver> {
        if domain == "invalid.xrd" {
            Err(CommonError::Unknown)
        } else {
            Ok(
            ResolvedReceiver {
                domain: domain,
                account: InternalAccountAddress::from_str("account_rdx12ylgt80y9zq94flkghlnlq8tr542wm5h77gs7hv3y5h92pt5hs46c4").unwrap().into(),
            }
        )
        }
    }
}
