use crate::prelude::*;

impl AccessRequirement {
    /// The simplest requirement with the same meaning, the form to show to
    /// users. In it:
    ///
    /// * `Anyone` and `NoOne` only appear on their own. A requirement that
    ///   needs no proof, e.g. "at least 0 of these", is `Anyone`, and one that
    ///   can never pass, e.g. "at least 3 of 2", is `NoOne`.
    /// * `AtLeast` has at least two requirements and a count between one and
    ///   their number. "At least 1 of" a single requirement is that
    ///   requirement.
    /// * "At least 1 of" doesn't directly contain another "at least 1 of", and
    ///   "all of" doesn't directly contain another "all of": they are merged.
    ///
    /// Duplicated requirements are kept, since each one counts: a group
    /// that lists a badge twice passes with fewer holders than it lists, see
    /// [`AccessRequirementForDisplay::has_overlapping_entries`].
    ///
    /// The result is `Anyone` exactly when the requirement passes without any
    /// proof, and `NoOne` exactly when no proofs can pass it. So an open or a
    /// locked requirement is never shown as anything else.
    pub fn simplified(self) -> Self {
        let Self::AtLeast {
            mut count,
            requirements,
        } = self
        else {
            return self;
        };

        let mut kept = Vec::new();
        for requirement in requirements.into_iter().map(Self::simplified) {
            match requirement {
                // Always counts, so one fewer is needed from the others.
                Self::Anyone => count = count.saturating_sub(1),
                // Never counts.
                Self::NoOne => {}
                requirement => kept.push(requirement),
            }
        }

        if count == 0 {
            return Self::Anyone;
        }
        if count as usize > kept.len() {
            return Self::NoOne;
        }
        if kept.len() == 1 {
            return kept.remove(0);
        }

        let is_any_of = count == 1;
        let is_all_of = count as usize == kept.len();
        let mut merged = Vec::new();
        for requirement in kept {
            match requirement {
                Self::AtLeast {
                    count: 1,
                    requirements,
                } if is_any_of => merged.extend(requirements),
                Self::AtLeast {
                    count: nested_count,
                    requirements,
                } if is_all_of
                    && nested_count as usize == requirements.len() =>
                {
                    count += nested_count - 1;
                    merged.extend(requirements);
                }
                requirement => merged.push(requirement),
            }
        }

        Self::AtLeast {
            count,
            requirements: merged,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;
    use proptest::test_runner::TestRunner;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = AccessRequirement;

    fn badges() -> [NonFungibleGlobalId; 3] {
        let resource = NonFungibleGlobalId::sample().resource_address;
        [1, 2, 3].map(|id| {
            NonFungibleGlobalId::new_unchecked(
                resource,
                NonFungibleLocalId::integer(id),
            )
        })
    }

    fn leaves() -> [SUT; 3] {
        badges().map(|badge| SUT::Badge {
            badge: ResourceOrNonFungible::NonFungible { value: badge },
        })
    }

    fn at_least(count: u32, requirements: impl Into<Vec<SUT>>) -> SUT {
        SUT::AtLeast {
            count,
            requirements: requirements.into(),
        }
    }

    /// Every auth zone signed by a subset of `badges()`, from none to all.
    fn auth_zones() -> Vec<AuthZone> {
        (0..1 << badges().len())
            .map(|subset: usize| AuthZone {
                implicit_non_fungibles: badges()
                    .into_iter()
                    .enumerate()
                    .filter(|(index, _)| subset & (1 << index) != 0)
                    .map(|(_, badge)| badge)
                    .collect(),
                proofs: vec![],
            })
            .collect()
    }

    /// Asserts that simplifying keeps the meaning in every auth zone, that the
    /// result has the simplified form, and that it is `Anyone` or `NoOne`
    /// exactly when the requirement is open or locked.
    fn assert_simplifies_correctly(requirement: &SUT, auth_zones: &[AuthZone]) {
        let simplified = requirement.clone().simplified();
        for auth_zone in auth_zones {
            assert_eq!(
                simplified.is_satisfied_by(auth_zone),
                requirement.is_satisfied_by(auth_zone),
                "{requirement:?} simplified to {simplified:?} changes its meaning for {auth_zone:?}"
            );
        }
        assert_simplified_form(&simplified, true);
        assert_eq!(simplified.clone().simplified(), simplified);

        let no_proofs = auth_zones.first().unwrap();
        let all_proofs = auth_zones.last().unwrap();
        assert_eq!(
            simplified == SUT::Anyone,
            requirement.is_satisfied_by(no_proofs),
            "{requirement:?} simplified to {simplified:?} hides whether it needs a proof"
        );
        assert_eq!(
            simplified == SUT::NoOne,
            !requirement.is_satisfied_by(all_proofs),
            "{requirement:?} simplified to {simplified:?} hides whether it can pass"
        );
    }

    fn assert_simplified_form(requirement: &SUT, is_root: bool) {
        let SUT::AtLeast {
            count,
            requirements,
        } = requirement
        else {
            assert!(
                is_root || !matches!(requirement, SUT::Anyone | SUT::NoOne),
                "{requirement:?} is nested"
            );
            return;
        };
        let len = requirements.len();
        assert!(len >= 2, "{requirement:?} has fewer than two requirements");
        assert!(
            *count >= 1 && *count as usize <= len,
            "{requirement:?} has a count outside 1..={len}"
        );
        for nested in requirements {
            if let SUT::AtLeast {
                count: nested_count,
                requirements: nested_requirements,
            } = nested
            {
                assert!(
                    !(*count == 1 && *nested_count == 1),
                    "{requirement:?} has a nested \"at least 1 of\""
                );
                assert!(
                    !(*count as usize == len
                        && *nested_count as usize == nested_requirements.len()),
                    "{requirement:?} has a nested \"all of\""
                );
            }
            assert_simplified_form(nested, false);
        }
    }

    /// Every `AtLeast` of up to `max_len` of `items`, with every count from 0
    /// to one more than the number of requirements.
    fn all_at_least(items: &[SUT], max_len: usize) -> Vec<SUT> {
        let mut lists: Vec<Vec<SUT>> = vec![vec![]];
        let mut last: Vec<Vec<SUT>> = vec![vec![]];
        for _ in 0..max_len {
            last = last
                .iter()
                .flat_map(|list| {
                    items.iter().map(move |item| {
                        let mut list = list.clone();
                        list.push(item.clone());
                        list
                    })
                })
                .collect();
            lists.extend(last.clone());
        }
        lists
            .into_iter()
            .flat_map(|list| {
                (0..=list.len() as u32 + 1)
                    .map(move |count| at_least(count, list.clone()))
            })
            .collect()
    }

    #[test]
    fn keeps_meaning_of_every_small_requirement() {
        let auth_zones = auth_zones();
        let [a, b, _] = leaves();
        let leaves = vec![SUT::Anyone, SUT::NoOne, a, b];

        let depth_one = all_at_least(&leaves, 3);
        for requirement in &depth_one {
            assert_simplifies_correctly(requirement, &auth_zones);
        }

        let narrow_depth_one =
            [leaves.clone(), all_at_least(&leaves, 2)].concat();
        for requirement in all_at_least(&narrow_depth_one, 2) {
            assert_simplifies_correctly(&requirement, &auth_zones);
        }
    }

    /// Requirements up to four levels deep over the three badges.
    fn requirements() -> impl Strategy<Value = SUT> {
        let leaves = leaves();
        let leaf = prop_oneof![
            Just(SUT::Anyone),
            Just(SUT::NoOne),
            (0usize..3).prop_map(move |index| leaves[index].clone()),
        ];
        leaf.prop_recursive(4, 64, 4, |nested| {
            (0u32..6, prop::collection::vec(nested, 0..5))
                .prop_map(|(count, requirements)| at_least(count, requirements))
        })
    }

    /// Random larger requirements. A failure is shrunk to a smallest
    /// requirement that still fails.
    #[test]
    fn keeps_meaning_of_random_requirements() {
        let auth_zones = auth_zones();
        TestRunner::new(ProptestConfig {
            cases: 20_000,
            failure_persistence: None,
            ..ProptestConfig::default()
        })
        .run(&requirements(), |requirement| {
            assert_simplifies_correctly(&requirement, &auth_zones);
            Ok(())
        })
        .unwrap();
    }
}
