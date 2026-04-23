use crate::prelude::*;

struct SecureStorageSessionStorage {
    secure_storage: SecureStorageClient,
}

#[async_trait::async_trait]
impl RadixConnectMobileSessionStorage for SecureStorageSessionStorage {
    async fn save_session(
        &self,
        session_id: SessionID,
        encoded_session: BagOfBytes,
    ) -> Result<()> {
        self.secure_storage
            .save_radix_connect_mobile_session(
                session_id.to_string(),
                encoded_session,
            )
            .await
    }

    async fn load_session(
        &self,
        session_id: SessionID,
    ) -> Result<Option<BagOfBytes>> {
        self.secure_storage
            .load_radix_connect_mobile_session(session_id.to_string())
            .await
    }
}

impl SargonOS {
    pub(crate) fn new_radix_connect_mobile(
        profile_state_holder: Arc<ProfileStateHolder>,
        networking_driver: Arc<dyn NetworkingDriver>,
        secure_storage: SecureStorageClient,
    ) -> Arc<RadixConnectMobile> {
        let relay_service_url_resolver = Arc::new(move || {
            profile_state_holder
                .current_relay_service()
                .map(|service| service.url.parsed())
        });

        Arc::new(RadixConnectMobile::new_with_relay_service_url_resolver(
            networking_driver,
            Arc::new(SecureStorageSessionStorage { secure_storage }),
            relay_service_url_resolver,
        ))
    }

    /// Returns the Radix Connect mobile client managed by this `SargonOS`.
    ///
    /// The client is created during `SargonOS::boot` and resolves relay URL from
    /// profile app preferences on each send operation.
    pub fn radix_connect_mobile(&self) -> Arc<RadixConnectMobile> {
        self.radix_connect_mobile.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Mutex;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = SargonOS;

    #[actix_rt::test]
    async fn radix_connect_mobile_is_eager_and_uses_updated_relay_without_recreation(
    ) {
        let requested_urls = Arc::new(Mutex::new(Vec::<Url>::new()));
        let requested_urls_spy = requested_urls.clone();
        let networking = Arc::new(MockNetworkingDriver::with_lazy_responses(
            move |request, _| {
                requested_urls_spy.lock().unwrap().push(request.url);
                NetworkResponse::new(200, Vec::<u8>::new())
            },
        ));
        let os = SUT::boot_test_with_networking_driver(networking)
            .await
            .unwrap();

        let mobile = os.radix_connect_mobile();
        assert!(Arc::ptr_eq(&mobile, &os.radix_connect_mobile()));

        let session = Session::sample();
        os.secure_storage
            .save_radix_connect_mobile_session(
                session.id.to_string(),
                session.serialize_to_bytes().unwrap().into(),
            )
            .await
            .unwrap();

        let first_relay_url = os.current_relay_service().unwrap().id();
        mobile
            .send_dapp_interaction_response(
                RadixConnectMobileWalletResponse::sample(),
            )
            .await
            .unwrap();

        os.change_current_relay_service(RelayService::sample_other())
            .await
            .unwrap();
        let second_relay_url = os.current_relay_service().unwrap().id();
        assert_ne!(first_relay_url, second_relay_url);

        mobile
            .send_dapp_interaction_response(
                RadixConnectMobileWalletResponse::sample(),
            )
            .await
            .unwrap();

        let urls = requested_urls.lock().unwrap().clone();
        assert_eq!(urls, vec![first_relay_url, second_relay_url]);
    }
}
