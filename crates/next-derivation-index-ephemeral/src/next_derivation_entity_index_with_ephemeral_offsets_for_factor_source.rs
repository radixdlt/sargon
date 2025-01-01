use crate::prelude::*;

/// Ephemeral / "Local" offsets, is a collection of counters with offset added
/// on top of next index analysis based on cache or profile. This is used so that
/// the FactorInstanceProvider can consecutively call `next` N times to get a range of
/// of `N` unique indices, added to the otherwise next based on cache/profile analysis.
#[derive(Debug, Default)]
pub struct NextDerivationEntityIndexWithEphemeralOffsetsForFactorSource {
    ephemeral_offsets: RwLock<HashMap<IndexAgnosticPath, HDPathComponent>>,
}

impl NextDerivationEntityIndexWithEphemeralOffsetsForFactorSource {
    /// Returns the next free index for the IndexAgnosticPath,
    /// and increases the local ephemeral offset.
    pub fn reserve(
        &self,
        agnostic_path: IndexAgnosticPath,
    ) -> Result<HDPathComponent> {
        let mut binding = self.ephemeral_offsets.write().unwrap();
        if let Some(existing) = binding.get_mut(&agnostic_path) {
            let free = *existing;
            let next = free.checked_add_one_to_global()?;
            *existing = next;
            Ok(free)
        } else {
            let free = HDPathComponent::from_local_key_space(
                0,
                agnostic_path.key_space,
            )?;
            let next = free.checked_add_one_to_global()?;
            binding.insert(agnostic_path, next);
            Ok(free)
        }
    }
}
