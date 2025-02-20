use profile_security_structures::prelude::TimePeriod;

use crate::prelude::*;

/// An entity ready for display and the delay required to wait until
/// confirming recovery is possible.
#[derive(Clone, Debug, PartialEq, Eq, std::hash::Hash)]
pub struct DelayedConfirmationForEntity {
    pub entity: EntityForDisplay,
    pub delay: TimePeriod,
}

impl DelayedConfirmationForEntity {
    pub fn new(entity: EntityForDisplay, delay: TimePeriod) -> Self {
        Self { entity, delay }
    }
}
impl HasSampleValues for DelayedConfirmationForEntity {
    fn sample() -> Self {
        Self::new(EntityForDisplay::sample(), TimePeriod::sample())
    }

    fn sample_other() -> Self {
        Self::new(EntityForDisplay::sample_other(), TimePeriod::sample_other())
    }
}

/// A list of entities which would fail in a transaction if we would
/// neglect certain factor source, either by user explicitly skipping
/// it or if implicitly neglected due to failure.
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
    pub entities_which_would_fail_auth: Vec<EntityForDisplay>,
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
        entities_which_would_fail_auth: impl IntoIterator<Item = EntityForDisplay>,
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

        assert!(!entities_which_would_require_delayed_confirmation.is_empty(), "'entities_which_would_require_delayed_confirmation' must not be empty, this type is not useful if it is empty.");

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

        assert!(entities_which_would_fail_auth.is_disjoint(
            &entities_which_would_require_delayed_confirmation
                .iter()
                .map(|d| d.entity)
                .collect::<IndexSet<_>>()
        ),);

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

    pub fn entities_which_would_fail_auth(&self) -> IndexSet<EntityForDisplay> {
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
            [DelayedConfirmationForEntity::sample_other()],
            [EntityForDisplay::sample()],
        )
    }

    fn sample_other() -> Self {
        Self::new(
            ID::sample_other(),
            [DelayedConfirmationForEntity::sample_other()],
            [EntityForDisplay::sample_other()],
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[allow(clippy::upper_case_acronyms)]
    type SUT = InvalidTransactionIfNeglected<TransactionIntentHash>;

    #[test]
    #[should_panic(
        expected = "'entities_which_would_fail_auth' must not be empty, this type is not useful if it is empty."
    )]
    fn panics_if_empty() {
        SUT::new(TransactionIntentHash::sample(), IndexSet::new());
    }

    #[test]
    #[should_panic(
        expected = "entities_which_would_fail_auth must not contain duplicates."
    )]
    fn panics_if_duplicates() {
        SUT::new(
            TransactionIntentHash::sample(),
            [],
            [EntityForDisplay::sample(), EntityForDisplay::sample()],
        );
    }

    #[test]
    fn new() {
        let would_fail =
            [EntityForDisplay::sample(), EntityForDisplay::sample_other()];
        let no_quick = [DelayedConfirmationForEntity::sample()];
        let sut = SUT::new(TransactionIntentHash::sample(), no_quick, entities);
        assert_eq!(
            sut.entities_which_would_fail_auth(),
            IndexSet::<_>::from_iter(entities.into_iter())
        );

        assert_eq!(
            sut.entities_which_would_require_delayed_confirmation(),
            IndexSet::<_>::from_iter(no_quick.into_iter())
        );
    }
}
