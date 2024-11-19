use crate::prelude::*;

pub trait HasMfaRole {
    fn mfa_role() -> RoleKind;
}

pub trait HasMfaRoleObjectSafe {
    fn get_mfa_role(&self) -> RoleKind;
}

impl<T: HasMfaRole> HasMfaRoleObjectSafe for T {
    fn get_mfa_role(&self) -> RoleKind {
        T::mfa_role()
    }
}
pub trait RoleWithFactors<Factor: std::cmp::Eq + std::hash::Hash> {
    fn get_threshold_factors(&self) -> &Vec<Factor>;
    fn get_threshold(&self) -> u8;
    fn get_override_factors(&self) -> &Vec<Factor>;

    fn all_factors(&self) -> Vec<&Factor> {
        let mut factors = self.get_threshold_factors().clone();
        factors.extend(self.get_override_factors().clone());
        factors
    }
}

pub trait HasFactorInstances {
    fn unique_factor_instances(&self) -> IndexSet<FactorInstance>;
}
