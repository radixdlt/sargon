use crate::prelude::*;

/// Essentially a map of `NextDerivationEntityIndexWithEphemeralOffsetsForFactorSource`
/// ephemeral offsets used by `NextDerivationEntityIndexAssigner`
/// to add ephemeral offsets to next index calculations.
#[derive(Default, Debug)]
pub struct NextDerivationEntityIndexWithEphemeralOffsets {
    ephemeral_offsets_per_factor_source: RwLock<
        HashMap<
            FactorSourceIDFromHash,
            NextDerivationEntityIndexWithEphemeralOffsetsForFactorSource,
        >,
    >,
}

impl NextDerivationEntityIndexWithEphemeralOffsets {
    /// Reserves the next ephemeral offset for `factor_source_id` for `agnostic_path`.
    /// Consecutive calls always returns a new value, which is `previous + 1` (given
    /// the same `factor_source_id, agnostic_path`)
    pub fn reserve(
        &self,
        factor_source_id: FactorSourceIDFromHash,
        agnostic_path: IndexAgnosticPath,
    ) -> Result<HDPathComponent> {
        let mut binding =
            self.ephemeral_offsets_per_factor_source.write().unwrap();
        if let Some(for_factor) = binding.get_mut(&factor_source_id) {
            for_factor.reserve(agnostic_path)
        } else {
            let new = NextDerivationEntityIndexWithEphemeralOffsetsForFactorSource::default();
            let next = new.reserve(agnostic_path)?;
            binding.insert(factor_source_id, new);
            Ok(next)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = NextDerivationEntityIndexWithEphemeralOffsets;

    #[test]
    fn test_contiguous() {
        let sut = SUT::default();
        let n = 4;
        let indices = (0..n)
            .map(|_| {
                sut.reserve(
                    FactorSourceIDFromHash::sample_at(0),
                    DerivationPreset::AccountVeci
                        .index_agnostic_path_on_network(NetworkID::Mainnet),
                )
                .unwrap()
            })
            .collect::<IndexSet<_>>();

        assert_eq!(
            indices,
            [0, 1, 2, 3]
                .into_iter()
                .map(|i| HDPathComponent::from_local_key_space(
                    i,
                    KeySpace::Unsecurified { is_hardened: true }
                )
                .unwrap())
                .collect::<IndexSet<_>>()
        );
    }

    #[test]
    fn test_zero_for_each_factor_sources_first_time() {
        let sut = SUT::default();
        let fsids = FactorSource::sample_all()
            .into_iter()
            .map(|f| f.id_from_hash())
            .collect_vec();
        let indices = fsids
            .clone()
            .into_iter()
            .map(|fsid| {
                sut.reserve(
                    fsid,
                    DerivationPreset::AccountVeci
                        .index_agnostic_path_on_network(NetworkID::Mainnet),
                )
                .unwrap()
            })
            .collect_vec();
        assert_eq!(
            indices,
            vec![
                HDPathComponent::Unsecurified(Unsecurified::Hardened(
                    UnsecurifiedHardened::ZERO
                ));
                fsids.len()
            ]
        );
    }

    #[test]
    fn test_zero_for_each_derivation_preset() {
        let sut = SUT::default();
        let derivation_presets = DerivationPreset::all();
        let indices = derivation_presets
            .clone()
            .into_iter()
            .map(|preset| {
                sut.reserve(
                    FactorSourceIDFromHash::sample_at(0),
                    preset.index_agnostic_path_on_network(NetworkID::Mainnet),
                )
                .unwrap()
            })
            .collect_vec();
        assert_eq!(
            indices,
            vec![
                HDPathComponent::unsecurified_hardened(0u32).unwrap(), // Account Veci
                HDPathComponent::Securified(SecurifiedU30::ZERO), // Account MFA
                HDPathComponent::Securified(SecurifiedU30::ZERO), // Account Rola
                HDPathComponent::unsecurified_hardened(0u32).unwrap(), // Identify Veci
                HDPathComponent::Securified(SecurifiedU30::ZERO), // Identity MFA
                HDPathComponent::Securified(SecurifiedU30::ZERO), // Identity Rola
            ]
        );
    }

    #[test]
    fn test_zero_for_each_network() {
        let sut = SUT::default();
        let network_ids = NetworkID::all();
        let indices = network_ids
            .clone()
            .into_iter()
            .map(|network_id| {
                sut.reserve(
                    FactorSourceIDFromHash::sample_at(0),
                    DerivationPreset::AccountMfa
                        .index_agnostic_path_on_network(network_id),
                )
                .unwrap()
            })
            .collect_vec();
        assert_eq!(
            indices,
            vec![HDPathComponent::Securified(SecurifiedU30::ZERO); 12]
        );
    }
}
