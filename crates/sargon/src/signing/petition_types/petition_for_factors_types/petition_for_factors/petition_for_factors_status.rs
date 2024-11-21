/// The status of building using a certain list of factors, e.g. threshold or
/// override factors list.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum PetitionForFactorsStatus {
    /// In progress, still gathering output from factors (signatures or public keys).
    InProgress,

    /// Finished building with factors, either successfully or failed.
    Finished(PetitionFactorsStatusFinished),
}

/// Finished building with factors, either successfully or failed.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum PetitionFactorsStatusFinished {
    /// Successful completion of building with factors.
    Success,

    /// Failure building with factors, either a simulated status, as in what
    /// would happen if we skipped a factor source, or a real failure, as in,
    /// the user explicitly chose to skip a factor source even though she was
    /// advised it would result in some transaction failing. Or we failed to
    /// use a required factor source for what some reason.
    Fail,
}

impl PetitionForFactorsStatus {
    /// Reduces / aggergates a list of `PetitionForFactorsStatus` into some
    /// other status, e.g. `PetitionsStatus`.
    pub(crate) fn aggregate<T>(
        statuses: impl IntoIterator<Item = Self>,
        valid: T,
        invalid: T,
        pending: T,
    ) -> T {
        let statuses = statuses.into_iter().collect::<Vec<_>>();

        let are_all_valid = statuses.iter().all(|s| {
            matches!(
                s,
                PetitionForFactorsStatus::Finished(
                    PetitionFactorsStatusFinished::Success
                )
            )
        });

        if are_all_valid {
            return valid;
        }

        let is_some_invalid = statuses.iter().any(|s| {
            matches!(
                s,
                PetitionForFactorsStatus::Finished(
                    PetitionFactorsStatusFinished::Fail
                )
            )
        });

        if is_some_invalid {
            return invalid;
        }

        pending
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = PetitionForFactorsStatus;
    use super::PetitionFactorsStatusFinished::*;
    use super::PetitionForFactorsStatus::*;

    #[test]
    fn aggregate_invalid() {
        let invalid = Some(1);
        let irrelevant = None;
        assert_eq!(
            SUT::aggregate(
                vec![InProgress, Finished(Fail), Finished(Success)],
                irrelevant,
                invalid,
                irrelevant
            ),
            invalid
        )
    }

    #[test]
    fn aggregate_pending() {
        let pending = Some(1);
        let irrelevant = None;
        assert_eq!(
            SUT::aggregate(
                vec![InProgress, Finished(Success), Finished(Success)],
                irrelevant,
                irrelevant,
                pending,
            ),
            pending
        )
    }

    #[test]
    fn aggregate_valid() {
        let valid = Some(1);
        let irrelevant = None;
        assert_eq!(
            SUT::aggregate(
                vec![Finished(Success), Finished(Success)],
                valid,
                irrelevant,
                irrelevant
            ),
            valid
        )
    }
}
