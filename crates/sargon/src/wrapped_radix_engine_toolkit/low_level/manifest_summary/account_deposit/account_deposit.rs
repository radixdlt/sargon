use crate::prelude::*;
use std::collections::HashMap;

#[derive(Clone, Debug, PartialEq, Eq, uniffi::Record)]
pub struct AccountDeposit {
    specified_resources: HashMap<ResourceAddress, SimpleResourceBounds>,
    unspecified_resources: UnspecifiedResources,
}

impl AccountDeposit {
    pub fn new(
        specified_resources: HashMap<ResourceAddress, SimpleResourceBounds>,
        unspecified_resources: UnspecifiedResources,
    ) -> Self {
        Self {
            specified_resources,
            unspecified_resources,
        }
    }
}

impl From<(ScryptoAccountDeposit, NetworkID)> for AccountDeposit {
    fn from(value: (ScryptoAccountDeposit, NetworkID)) -> Self {
        let (scrypto_value, network_id) = value;
        let specified_resources = scrypto_value
            .specified_resources()
            .into_iter()
            .map(|(address, bounds)| {
                ((address.clone(), network_id).into(), bounds.clone().into())
            })
            .collect();
        let unspecified_resources =
            scrypto_value.unspecified_resources().into();
        Self::new(specified_resources, unspecified_resources)
    }
}

impl HasSampleValues for AccountDeposit {
    fn sample() -> Self {
        Self::new(
            vec![(ResourceAddress::sample(), SimpleResourceBounds::sample())]
                .into_iter()
                .collect(),
            UnspecifiedResources::sample(),
        )
    }

    fn sample_other() -> Self {
        Self::new(
            vec![(
                ResourceAddress::sample_sim_xrd(),
                SimpleResourceBounds::sample(),
            )]
            .into_iter()
            .collect(),
            UnspecifiedResources::NonePresent,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = AccountDeposit;

    #[test]
    fn equality() {
        assert_eq!(SUT::sample(), SUT::sample());
        assert_eq!(SUT::sample_other(), SUT::sample_other());
    }

    #[test]
    fn inequality() {
        assert_ne!(SUT::sample(), SUT::sample_other());
    }

    #[test]
    fn new_account_deposit() {
        let specified_resources: HashMap<
            ResourceAddress,
            SimpleResourceBounds,
        > = vec![
            (ResourceAddress::sample(), SimpleResourceBounds::sample()),
            (
                ResourceAddress::sample_other(),
                SimpleResourceBounds::sample_other(),
            ),
        ]
        .into_iter()
        .collect();
        let unspecified_resources = UnspecifiedResources::sample();

        let account_deposit = AccountDeposit::new(
            specified_resources.clone(),
            unspecified_resources.clone(),
        );

        assert_eq!(account_deposit.specified_resources, specified_resources);
        assert_eq!(
            account_deposit.unspecified_resources,
            unspecified_resources
        );
    }
}
