use crate::prelude::*;

impl Profile {
    pub fn factor_source_by_id<F>(&self, id: &FactorSourceID) -> Result<F>
    where
        F: IsFactorSource,
    {
        self.factor_sources
            .get(id)
            .ok_or(CommonError::ProfileDoesNotContainFactorSourceWithID)
            .and_then(|f| {
                f.clone()
                    .try_into()
                    .map_err(|_| CommonError::CastFactorSourceWrongKind {
                        expected: F::factor_source_kind(),
                        found: f.factor_source_kind(),
                    })
            })
    }

    pub fn device_factor_source_by_id(
        &self,
        id: &FactorSourceIDFromHash,
    ) -> Result<DeviceFactorSource> {
        self.factor_source_by_id(&id.clone().into())
    }

    pub fn bdfs(&self) -> DeviceFactorSource {
        let device_factor_source = self
            .factor_sources
            .clone()
            .into_iter()
            .map(|f| f.as_device().cloned())
            .filter_map(std::convert::identity)
            .collect_vec();

        let explicit_main =
            device_factor_source
                .clone()
                .into_iter()
                .filter(|x| x.is_main_bdfs())
                .collect_vec()
                .first()
                .cloned();

        let implicit_main = device_factor_source
            .into_iter()
            .filter(|x| x.common.supports_babylon())
            .collect_vec()
            .first()
            .expect("A Profile should always contain Babylon DeviceFactorSource")
            .clone();

        return explicit_main.unwrap_or(implicit_main).clone();
    }

    pub fn next_derivation_index_for_entity(
        &self,
        kind: EntityKind,
        network_id: NetworkID,
    ) -> HDPathValue {
        match kind {
            EntityKind::Persona => panic!("Personas are not supported yet"),
            EntityKind::Accounts => {}
        };
        let index = self
            .networks
            .get(&network_id)
            .map(|n| {
                n.accounts
                    .items()
                    .into_iter()
                    .filter(|a| match &a.security_state {
                        EntitySecurityState::Unsecured { value } => {
                            value.transaction_signing.factor_source_id == self.bdfs().id
                        }
                    })
                    .collect_vec()
                    .len()
            })
            .unwrap_or(0);

        return index as HDPathValue;
    }
}
