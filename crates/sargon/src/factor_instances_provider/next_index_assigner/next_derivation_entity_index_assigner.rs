use crate::prelude::*;

/// An assigner of derivation entity indices, used by the FactorInstancesProvider
/// to map `IndexAgnosticPath` -> `DerivationPath` for some FactorSource on
/// some NetworkID.
///
/// This assigner works with the:
/// * cache
/// * profile
/// * local offsets
///
/// More specifically the assigner's `next` method performs approximately this
/// operation:
///
/// ```ignore
/// pub fn next(
///    &mut self,
///    fs_id: FactorSourceIDFromHash,
///    path: IndexAgnosticPath,
/// ) -> Result<HDPathComponent> {
///     let next_from_cache = self.cache_analyzing.next(fs_id, path).unwrap_or(0);
///     let next_from_profile = self.profile_analyzing.next(fs_id, path).unwrap_or(0);
///     
///     let max_index = std::cmp::max(next_from_profile, next_from_cache);
///     let ephemeral_offset = self.ephemeral_offsets.reserve()?;
///
///     max_index + ephemeral_offset
/// ```
pub struct NextDerivationEntityIndexAssigner {
    profile_analyzing: NextDerivationEntityIndexProfileAnalyzingAssigner,
    cache_analyzing: NextDerivationEntityIndexCacheAnalyzingAssigner,
    ephemeral_offsets: NextDerivationEntityIndexWithEphemeralOffsets,
}

impl NextDerivationEntityIndexAssigner {
    pub fn new(
        network_id: NetworkID,
        profile: impl Into<Option<Arc<Profile>>>,
        cache: FactorInstancesCache,
    ) -> Self {
        let profile_analyzing =
            NextDerivationEntityIndexProfileAnalyzingAssigner::new(
                network_id, profile,
            );
        let cache_analyzing =
            NextDerivationEntityIndexCacheAnalyzingAssigner::new(cache);
        let ephemeral_offsets =
            NextDerivationEntityIndexWithEphemeralOffsets::default();
        Self {
            profile_analyzing,
            cache_analyzing,
            ephemeral_offsets,
        }
    }

    /// Returns the next index for the given `FactorSourceIDFromHash` and
    /// `IndexAgnosticPath`, by analyzing the cache, the profile and adding
    /// local ephemeral offsets.
    pub fn next(
        &self,
        factor_source_id: FactorSourceIDFromHash,
        index_agnostic_path: IndexAgnosticPath,
    ) -> Result<HDPathComponent> {
        let default_index = HDPathComponent::from_local_key_space(
            0u32,
            index_agnostic_path.key_space,
        )?;

        let maybe_next_from_cache = self
            .cache_analyzing
            .next(factor_source_id, index_agnostic_path)?;

        let next_from_cache = maybe_next_from_cache.unwrap_or(default_index);
        let ephemeral = self
            .ephemeral_offsets
            .reserve(factor_source_id, index_agnostic_path)?;

        let maybe_next_from_profile = self
            .profile_analyzing
            .next(factor_source_id, index_agnostic_path)?;

        let next_from_profile =
            maybe_next_from_profile.unwrap_or(default_index);

        let max_index = std::cmp::max(next_from_profile, next_from_cache);

        max_index
            // We add the LOCAL index "offset" to the max_index
            .checked_add_n_to_global(u32::from(
                ephemeral.index_in_local_key_space(),
            ))
    }
}
