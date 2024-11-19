use crate::prelude::*;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct FactorSourcesOfKind {
    pub(crate) kind: FactorSourceKind,
    factor_sources: Vec<FactorSource>,
}

impl FactorSourcesOfKind {
    pub(crate) fn new(
        kind: FactorSourceKind,
        factor_sources: impl IntoIterator<Item = FactorSource>,
    ) -> Result<Self> {
        let factor_sources =
            factor_sources.into_iter().collect::<IndexSet<_>>();
        if factor_sources.is_empty() {
            return Err(CommonError::FactorSourcesOfKindEmptyFactors);
        }

        if let Some(factor_source) = factor_sources
            .iter()
            .find(|f| f.get_factor_source_kind() != kind)
        {
            return Err(CommonError::InvalidFactorSourceKind {
                bad_value: factor_source.get_factor_source_kind().to_string(),
            });
        }

        Ok(Self {
            kind,
            factor_sources: factor_sources.into_iter().collect(),
        })
    }

    pub(crate) fn factor_sources(&self) -> IndexSet<FactorSource> {
        self.factor_sources.clone().into_iter().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[allow(clippy::upper_case_acronyms)]
    type SUT = FactorSourcesOfKind;

    #[test]
    fn invalid_empty() {
        assert_eq!(
            SUT::new(FactorSourceKind::Device, []),
            Err(CommonError::FactorSourcesOfKindEmptyFactors)
        );
    }

    #[test]
    fn invalid_single_element() {
        assert_eq!(
            SUT::new(
                FactorSourceKind::Device,
                [FactorSource::sample_arculus()]
            ),
            Err(CommonError::InvalidFactorSourceKind {
                bad_value: FactorSourceKind::ArculusCard.to_string()
            })
        );
    }

    #[test]
    fn invalid_two_two() {
        assert_eq!(
            SUT::new(
                FactorSourceKind::Device,
                [
                    FactorSource::sample_arculus(),
                    FactorSource::sample_device_babylon(),
                    FactorSource::sample_arculus_other(),
                    FactorSource::sample_device_babylon_other()
                ]
            ),
            Err(CommonError::InvalidFactorSourceKind {
                bad_value: FactorSourceKind::ArculusCard.to_string()
            })
        );
    }

    #[test]
    fn valid_one() {
        let sources =
            IndexSet::<FactorSource>::just(FactorSource::sample_device());
        let sut = SUT::new(FactorSourceKind::Device, sources.clone()).unwrap();
        assert_eq!(sut.factor_sources(), sources);
    }

    #[test]
    fn valid_two() {
        let sources = IndexSet::<FactorSource>::from_iter([
            FactorSource::sample_ledger(),
            FactorSource::sample_ledger_other(),
        ]);
        let sut =
            SUT::new(FactorSourceKind::LedgerHQHardwareWallet, sources.clone())
                .unwrap();
        assert_eq!(sut.factor_sources(), sources);
        assert_eq!(sut.factor_sources().len(), 2);
    }

    #[test]
    fn valid_no_duplicates() {
        let sources = IndexSet::<FactorSource>::from_iter([
            FactorSource::sample_ledger(),
            FactorSource::sample_ledger(),
        ]);
        let sut =
            SUT::new(FactorSourceKind::LedgerHQHardwareWallet, sources.clone())
                .unwrap();
        assert_eq!(sut.factor_sources(), sources);
        assert_eq!(sut.factor_sources().len(), 1);
    }
}
