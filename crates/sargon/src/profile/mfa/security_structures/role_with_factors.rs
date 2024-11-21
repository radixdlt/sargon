use crate::prelude::*;

pub trait RoleWithFactors<Factor: std::cmp::Eq + std::hash::Hash> {
    fn get_threshold_factors(&self) -> &Vec<Factor>;
    fn get_threshold(&self) -> u8;
    fn get_override_factors(&self) -> &Vec<Factor>;

    fn all_factors(&self) -> IndexSet<&Factor> {
        let mut factors =
            IndexSet::from_iter(self.get_threshold_factors().iter());
        factors.extend(self.get_override_factors().iter());
        factors
    }
}

pub trait HasFactorInstances {
    fn unique_factor_instances(&self) -> IndexSet<FactorInstance>;
}
