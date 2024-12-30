use crate::prelude::*;

impl Profile {
    // TODO: Sometimes later it would be nice to remove this method
    // and only use `diagnostics_for_factor_instances_valid_with_handler` and then
    // send a handler from SargonOS which has access to some new driver which
    // can use Swift Issue Reporting API:
    // https://github.com/pointfreeco/swift-issue-reporting
    // which will cause execution to halt with a runtime issue, which will be great
    // for debugging and finding issues!
    // Maybe android host can raise an exception..?
    pub(crate) fn diagnostics_for_factor_instances_valid(&self) {
        self.diagnostics_for_factor_instances_valid_with_handler(|_| {});
    }

    pub(crate) fn diagnostics_for_factor_instances_valid_with_handler(
        &self,
        mut on_duplicate: impl FnMut(DuplicateInstances),
    ) {
        let Some(duplicate_instances) = self.check_for_duplicated_instances()
        else {
            return;
        };

        error!("Duplicated FactorInstances found {:?}", duplicate_instances);
        on_duplicate(duplicate_instances);
    }
}
