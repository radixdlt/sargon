use crate::prelude::*;

#[derive(Debug)]
pub struct UnsafeStorageClient {
    #[allow(dead_code)]
    driver: Arc<dyn UnsafeStorageDriver>,
}

impl UnsafeStorageClient {
    pub(crate) fn new(driver: Arc<dyn UnsafeStorageDriver>) -> Self {
        Self { driver }
    }
}

impl UnsafeStorageClient {
    //======
    // Save T
    //======
    pub async fn save<T>(&self, key: UnsafeStorageKey, value: &T) -> Result<()>
    where
        T: serde::Serialize,
    {
        let json = serde_json::to_vec(value)
            .map_err(|_| CommonError::FailedToSerializeToJSON)?;
        self.driver
            .save_data(key, BagOfBytes::from(json))
            // tarpaulin will incorrectly flag next line is missed
            .await
    }

    //======
    // Load T
    //======
    /// Loads bytes from UnsafeStorageDriver and deserializes them into `T`.
    ///
    /// Returns `Ok(None)` if no bytes were found, returns Err if failed
    /// to load bytes or failed to deserialize the JSON into a `T`.
    pub async fn load<T>(&self, key: UnsafeStorageKey) -> Result<Option<T>>
    where
        T: for<'a> serde::Deserialize<'a>,
    {
        self.driver.load_data(key).await.and_then(|o| match o {
            None => Ok(None),
            Some(j) => serde_json::from_slice(j.as_slice())
                .map_failed_to_deserialize_bytes::<T>(j.as_slice()),
        })
    }

    /// Loads bytes from UnsafeStorageDriver and deserializes them into `T`.
    ///
    /// Returns Err if failed to load bytes or failed to deserialize the JSON into a `T`,
    /// unlike `load` this method returns `default` if `None` bytes were found.
    pub async fn load_unwrap_or<T>(
        &self,
        key: UnsafeStorageKey,
        default: T,
    ) -> T
    where
        T: for<'a> serde::Deserialize<'a> + Clone,
    {
        self.load(key)
            .await
            .map(|o| o.unwrap_or(default.clone()))
            .unwrap_or(default)
    }

    //======
    // Backup up mnemonics
    //======
    pub async fn check_if_mnemonic_is_backed_up(
        &self,
        factor_source: DeviceFactorSource,
    ) -> Result<bool> {
        let result: Vec<FactorSourceIDFromHash> = self
            .load_unwrap_or(
                UnsafeStorageKey::FactorSourceUserHasWrittenDown,
                Vec::new(),
            )
            .await;
        Ok(result.contains(&factor_source.id))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_sut() -> UnsafeStorageClient {
        let driver = EphemeralUnsafeStorage::new();
        UnsafeStorageClient::new(driver)
    }

    #[actix_rt::test]
    async fn load_ok_when_none() {
        let sut = make_sut();
        assert_eq!(
            sut.load::<String>(
                UnsafeStorageKey::FactorSourceUserHasWrittenDown
            )
            .await,
            Ok(None)
        );
    }

    #[actix_rt::test]
    async fn load_successful() {
        let sut = make_sut();

        let value = String::sample();
        assert!(sut
            .save(UnsafeStorageKey::FactorSourceUserHasWrittenDown, &value)
            .await
            .is_ok());
        assert_eq!(
            sut.load::<String>(
                UnsafeStorageKey::FactorSourceUserHasWrittenDown
            )
            .await,
            Ok(Some(value))
        );
    }

    #[actix_rt::test]
    async fn load_unwrap_or_some_default_not_used() {
        let sut = make_sut();

        let value = String::sample();
        let default = String::sample_other();
        assert!(sut
            .save(UnsafeStorageKey::FactorSourceUserHasWrittenDown, &value)
            .await
            .is_ok());
        assert_eq!(
            sut.load_unwrap_or::<String>(
                UnsafeStorageKey::FactorSourceUserHasWrittenDown,
                default
            )
            .await,
            value
        );
    }

    #[actix_rt::test]
    async fn load_unwrap_or_none_default_is_used() {
        let sut = make_sut();

        assert_eq!(
            sut.load_unwrap_or::<String>(
                UnsafeStorageKey::FactorSourceUserHasWrittenDown,
                String::sample_other()
            )
            .await,
            String::sample_other()
        );
    }

    #[actix_rt::test]
    async fn check_if_mnemonic_is_backed_up() {
        let sut = make_sut();
        let factor_source = DeviceFactorSource::sample();

        // Check it isn't initially backed up
        assert!(!sut
            .check_if_mnemonic_is_backed_up(factor_source.clone())
            .await
            .unwrap());

        // Backup the mnemonic
        assert!(sut
            .save(
                UnsafeStorageKey::FactorSourceUserHasWrittenDown,
                &vec![factor_source.id]
            )
            .await
            .is_ok());

        // Check it is now backed up
        assert!(sut
            .check_if_mnemonic_is_backed_up(factor_source.clone())
            .await
            .unwrap());

        // Check another factor source isn't backed up
        assert!(!sut
            .check_if_mnemonic_is_backed_up(DeviceFactorSource::sample_other())
            .await
            .unwrap());
    }
}
