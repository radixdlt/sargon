use crate::prelude::*;

pub trait OsNewFactorAddingManagerFactory {
    fn make_device_factor_adding_manager(&self) -> DeviceFactorAddingManager;
}

impl OsNewFactorAddingManagerFactory for Arc<SargonOS> {
    fn make_device_factor_adding_manager(&self) -> DeviceFactorAddingManager {
        let os_new_factor_adding: Arc<dyn OsNewFactorAdding> =
            Arc::new(self.clone());
        DeviceFactorAddingManager::new(os_new_factor_adding)
    }
}
