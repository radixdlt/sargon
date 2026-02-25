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
    /// Returns a Radix Connect mobile client managed by this `SargonOS`.
    ///
    /// The client resolves relay URL lazily from profile app preferences on each
    /// send operation, so hosts do not need to recreate it after relay changes.
    pub fn radix_connect_mobile(self: &Arc<Self>) -> Arc<RadixConnectMobile> {
        if let Ok(guard) = self.radix_connect_mobile.read()
            && let Some(existing) = guard.clone()
        {
            return existing;
        }

        let weak_os = Arc::downgrade(self);
        let relay_service_url_resolver = Arc::new(move || {
            weak_os
                .upgrade()
                .ok_or(CommonError::Unknown {
                    error_message:
                        "SargonOS was dropped before relay URL resolution"
                            .to_owned(),
                })?
                .current_relay_service()
                .map(|service| service.url)
        });

        let mobile =
            Arc::new(RadixConnectMobile::new_with_relay_service_url_resolver(
                self.http_client.driver.clone(),
                Arc::new(SecureStorageSessionStorage {
                    secure_storage: self.secure_storage.clone(),
                }),
                relay_service_url_resolver,
            ));

        if let Ok(mut guard) = self.radix_connect_mobile.write() {
            *guard = Some(mobile.clone());
        }

        mobile
    }
}
