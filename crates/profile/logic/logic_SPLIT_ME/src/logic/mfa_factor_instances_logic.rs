use crate::prelude::*;

pub trait MFAFactorInstancesUpdating {
    fn insert_mfa_factor_instances(
        &mut self,
        network_id: NetworkID,
        mfa_factor_instances: MFAFactorInstances,
    );
}

impl MFAFactorInstancesUpdating for ProfileNetworks {
    fn insert_mfa_factor_instances(
        &mut self,
        network_id: NetworkID,
        mfa_factor_instances: MFAFactorInstances,
    ) {
        self.update_with(network_id, |n| {
            n.mfa_factor_instances.extend(mfa_factor_instances.clone())
        });
    }
}
