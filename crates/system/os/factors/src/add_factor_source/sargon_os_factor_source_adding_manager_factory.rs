use crate::prelude::*;

pub trait OsFactorSourceAddingManagerFactory {
    fn make_device_factor_source_adding_manager(
        &self,
    ) -> DeviceFactorSourceAddingManager;
}

impl OsFactorSourceAddingManagerFactory for Arc<SargonOS> {
    fn make_device_factor_source_adding_manager(
        &self,
    ) -> DeviceFactorSourceAddingManager {
        let os_new_factor_adding: Arc<dyn OsFactorSourceAdder> =
            Arc::new(self.clone());
        DeviceFactorSourceAddingManager::new(os_new_factor_adding)
    }
}
