use profile_logic::SecurityStructureMetadata;
use profile_security_structures::prelude::TimePeriod;

use crate::prelude::*;

/// An entity ready for display and the delay required to wait until
/// confirming recovery is possible.
///
/// Only relevant for Securified entities.
#[derive(Clone, Debug, PartialEq, Eq, std::hash::Hash)]
pub struct DelayedConfirmationForEntity {
    pub entity_for_display: EntityForDisplay,

    /// N.B. this value is read out from the **committed** shield, not
    /// the shield which is being applied.
    pub delay: TimePeriod,

    /// N.B. this is the metadata of the **provisional** shield, not the
    /// committed shield.
    pub shield_metadata: SecurityStructureMetadata,
}

impl DelayedConfirmationForEntity {
    pub fn new(
        entity_for_display: EntityForDisplay,
        delay: TimePeriod,
        shield_metadata: SecurityStructureMetadata,
    ) -> Self {
        Self {
            entity_for_display,
            delay,
            shield_metadata,
        }
    }
}
impl HasSampleValues for DelayedConfirmationForEntity {
    fn sample() -> Self {
        Self::new(
            EntityForDisplay::sample(),
            TimePeriod::sample(),
            SecurityStructureMetadata::sample(),
        )
    }

    fn sample_other() -> Self {
        Self::new(
            EntityForDisplay::sample_other(),
            TimePeriod::sample_other(),
            SecurityStructureMetadata::sample_other(),
        )
    }
}

#[derive(Clone, Debug, PartialEq, Eq, std::hash::Hash)]
pub struct InvalidTransactionForEntity {
    /// The entity in the transaction which would fail.
    pub entity_for_display: EntityForDisplay,
    /// None for transactions which are not shield applications
    /// Some for transactions which are shield applications.
    pub shield_metadata: Option<SecurityStructureMetadata>,
}

impl InvalidTransactionForEntity {
    pub fn new(
        entity_for_display: EntityForDisplay,
        shield_metadata: impl Into<Option<SecurityStructureMetadata>>,
    ) -> Self {
        Self {
            entity_for_display,
            shield_metadata: shield_metadata.into(),
        }
    }
}
impl HasSampleValues for InvalidTransactionForEntity {
    fn sample() -> Self {
        Self::new(
            EntityForDisplay::sample(),
            SecurityStructureMetadata::sample(),
        )
    }

    fn sample_other() -> Self {
        Self::new(EntityForDisplay::sample_other(), None)
    }
}

/// A list of entities which would fail in a transaction if we would
/// neglect certain factor source, either by user explicitly skipping
/// it or if implicitly neglected due to failure.
///
/// And a list of entities which would require delayed confirmation
/// - i.e. quick confirmation is not possible.
#[derive(Clone, Debug, PartialEq, Eq, std::hash::Hash)]
pub struct InvalidTransactionIfNeglected<ID: SignableID> {
    /// The intent hash of the transaction which would be invalid if a
    /// certain factor source would be neglected, either if user
    /// explicitly skipped it or implicitly neglected due to failure.
    pub signable_id: ID,

    /// Entities which would require delayed confirmation - i.e.
    /// quick confirmation is not possible.
    pub entities_which_would_require_delayed_confirmation:
        Vec<DelayedConfirmationForEntity>,

    /// The entities in the transaction which would fail auth.
    ///
    /// This is either used for transactions not relating to
    /// applications of security shields or for transactions
    /// which **are** applications of security shields and with
    /// the state of being unable to make the transaction valid -
    /// e.g. neither Primary nor Recovery role possible to exercise.
    pub entities_which_would_fail_auth: Vec<InvalidTransactionForEntity>,
}

impl<ID: SignableID> InvalidTransactionIfNeglected<ID> {
    /// Constructs a new `InvalidTransactionIfNeglected` from an IndexSet of
    /// entities which would fail auth..
    ///
    /// # Panics
    /// Panics if `entities_which_would_fail_auth` is empty.
    ///
    /// Panics if any of the entities in `entities_which_would_fail_auth` are also
    /// in `entities_which_would_require_delayed_confirmation`.
    pub fn new(
        signable_id: ID,
        entities_which_would_require_delayed_confirmation: impl IntoIterator<
            Item = DelayedConfirmationForEntity,
        >,
        entities_which_would_fail_auth: impl IntoIterator<
            Item = InvalidTransactionForEntity,
        >,
    ) -> Self {
        let entities_which_would_require_delayed_confirmation =
            entities_which_would_require_delayed_confirmation
                .into_iter()
                .collect_vec();
        let len = entities_which_would_require_delayed_confirmation.len();
        let entities_which_would_require_delayed_confirmation =
            entities_which_would_require_delayed_confirmation
                .into_iter()
                .collect::<IndexSet<_>>();

        assert_eq!(
            entities_which_would_require_delayed_confirmation.len(),
            len,
            "entities_which_would_require_delayed_confirmation must not contain duplicates."
        );

        let entities_which_would_fail_auth =
            entities_which_would_fail_auth.into_iter().collect_vec();
        let len = entities_which_would_fail_auth.len();
        let entities_which_would_fail_auth = entities_which_would_fail_auth
            .into_iter()
            .collect::<IndexSet<_>>();

        assert!(!entities_which_would_fail_auth.is_empty(), "'entities_which_would_fail_auth' must not be empty, this type is not useful if it is empty.");

        assert_eq!(
            entities_which_would_fail_auth.len(),
            len,
            "entities_which_would_fail_auth must not contain duplicates."
        );

        let a = entities_which_would_fail_auth
            .iter()
            .map(|e| e.entity_for_display)
            .collect::<IndexSet<_>>();
        let b = entities_which_would_require_delayed_confirmation
            .iter()
            .map(|d| d.entity_for_display)
            .collect::<IndexSet<_>>();
        let intersection = a.intersection(&b).collect_vec();
        if !intersection.is_empty() {
            panic!("Entities found both in entities_which_would_fail_auth and entities_which_would_require_delayed_confirmation - this is invalid and can be considered a programmer error. Shared entities: {:?}", intersection);
        }

        Self {
            signable_id,
            entities_which_would_require_delayed_confirmation:
                entities_which_would_require_delayed_confirmation
                    .into_iter()
                    .collect_vec(),
            entities_which_would_fail_auth: entities_which_would_fail_auth
                .into_iter()
                .collect_vec(),
        }
    }

    pub fn entities_which_would_fail_auth(
        &self,
    ) -> IndexSet<InvalidTransactionForEntity> {
        IndexSet::from_iter(self.entities_which_would_fail_auth.clone())
    }

    pub fn entities_which_would_require_delayed_confirmation(
        &self,
    ) -> IndexSet<DelayedConfirmationForEntity> {
        IndexSet::from_iter(
            self.entities_which_would_require_delayed_confirmation
                .clone(),
        )
    }
}

impl<ID: SignableID + HasSampleValues> HasSampleValues
    for InvalidTransactionIfNeglected<ID>
{
    fn sample() -> Self {
        Self::new(
            ID::sample(),
            [DelayedConfirmationForEntity::sample()],
            [InvalidTransactionForEntity::sample_other()],
        )
    }

    fn sample_other() -> Self {
        Self::new(
            ID::sample_other(),
            [DelayedConfirmationForEntity::sample_other()],
            [InvalidTransactionForEntity::sample()],
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[allow(clippy::upper_case_acronyms)]
    type SUT = InvalidTransactionIfNeglected<TransactionIntentHash>;

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
    #[should_panic(
        expected = "'entities_which_would_fail_auth' must not be empty, this type is not useful if it is empty."
    )]
    fn panics_if_empty() {
        SUT::new(TransactionIntentHash::sample(), [], []);
    }

    #[test]
    #[should_panic(
        expected = "entities_which_would_fail_auth must not contain duplicates."
    )]
    fn panics_if_duplicates() {
        SUT::new(
            TransactionIntentHash::sample(),
            [],
            [
                InvalidTransactionForEntity::sample(),
                InvalidTransactionForEntity::sample(),
            ],
        );
    }

    #[test]
    fn new() {
        let would_fail = [InvalidTransactionForEntity::sample_other()];
        let no_quick = [DelayedConfirmationForEntity::sample()];
        let sut = SUT::new(
            TransactionIntentHash::sample(),
            no_quick.clone(),
            would_fail.clone(),
        );
        assert_eq!(
            sut.entities_which_would_fail_auth(),
            IndexSet::<_>::from_iter(would_fail.into_iter())
        );

        assert_eq!(
            sut.entities_which_would_require_delayed_confirmation(),
            IndexSet::<_>::from_iter(no_quick.into_iter())
        );
    }
}
