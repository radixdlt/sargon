use crate::prelude::*;

/// Represents an account deposit, which includes specified and unspecified resources.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AccountDeposits {
    pub specified_resources: Vec<SimpleResourceBounds>,
    pub unspecified_resources: UnspecifiedResources,
}

impl AccountDeposits {
    fn new(
        specified_resources: Vec<SimpleResourceBounds>,
        unspecified_resources: UnspecifiedResources,
    ) -> Self {
        Self {
            specified_resources,
            unspecified_resources,
        }
    }
}

#[cfg(debug_assertions)]
impl AccountDeposits {
    pub fn new_for_test(
        specified_resources: Vec<SimpleResourceBounds>,
        unspecified_resources: UnspecifiedResources,
    ) -> Self {
        Self::new(specified_resources, unspecified_resources)
    }
}

impl From<(Vec<ScryptoAccountDeposit>, NetworkID)> for AccountDeposits {
    fn from(value: (Vec<ScryptoAccountDeposit>, NetworkID)) -> Self {
        let (deposits, network_id) = value;
        // flatten the specified resources into a vec
        let specified_resources = deposits
            .iter()
            .flat_map(|deposit| {
                deposit.specified_resources().into_iter().map(
                    |(address, bounds)| {
                        SimpleResourceBounds::from((
                            (*address, network_id).into(),
                            bounds.clone(),
                        ))
                    },
                )
            })
            .collect();

        // fold the unspecified resources into a single value.
        let unspecified_resources = deposits
            .iter()
            .map(|deposit| deposit.unspecified_resources())
            .fold(ScryptoUnspecifiedResources::NonePresent, |acc, next| {
                acc.add(next)
            });

        Self::new(specified_resources, unspecified_resources.into())
    }
}

impl HasSampleValues for AccountDeposits {
    fn sample() -> Self {
        Self::new(Vec::<_>::sample(), UnspecifiedResources::sample())
    }

    fn sample_other() -> Self {
        Self::new(
            Vec::<_>::sample_other(),
            UnspecifiedResources::sample_other(),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = AccountDeposits;

    #[test]
    fn equality() {
        assert_eq!(SUT::sample(), SUT::sample());
        assert_eq!(SUT::sample_other(), SUT::sample_other());
    }

    #[test]
    fn inequality() {
        assert_ne!(SUT::sample(), SUT::sample_other());
    }
}
