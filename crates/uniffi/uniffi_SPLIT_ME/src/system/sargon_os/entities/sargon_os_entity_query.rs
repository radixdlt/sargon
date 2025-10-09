use crate::prelude::*;

#[uniffi::export]
impl SargonOS {
    fn entity_by_access_controller_address(
        &self,
        address: AccessControllerAddress,
    ) -> Result<AccountOrPersona> {
        self.wrapped
            .entity_by_access_controller_address(address.into_internal())
            .into_result()
    }
}
