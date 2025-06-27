use crate::prelude::*;

pub trait SecurityStructureQuerying {
    fn get_main_security_structure(
        &self,
    ) -> Option<SecurityStructureOfFactorSourceIds>;
}

impl SecurityStructureQuerying for Security {
    fn get_main_security_structure(
        &self,
    ) -> Option<SecurityStructureOfFactorSourceIds> {
        self.security_structures_of_factor_source_ids
            .iter()
            .find(|s| s.metadata.is_main())
    }
}
