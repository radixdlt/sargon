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
    /// Duplicated requirements are kept, since each one counts.
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

    fn leaf(index: usize) -> SUT {
        leaves()[index].clone()
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
    fn merges_nested_any_of_and_all_of() {
        assert_eq!(
            at_least(1, [leaf(0), at_least(1, [leaf(1), leaf(2)])])
                .simplified(),
            at_least(1, [leaf(0), leaf(1), leaf(2)])
        );
        assert_eq!(
            at_least(2, [leaf(0), at_least(2, [leaf(1), leaf(2)])])
                .simplified(),
            at_least(3, [leaf(0), leaf(1), leaf(2)])
        );
        // "2 of 3" inside "at least 1 of" has a different meaning, so it stays.
        let threshold = at_least(2, [leaf(0), leaf(1), leaf(2)]);
        assert_eq!(
            at_least(1, [threshold.clone(), leaf(0)]).simplified(),
            at_least(1, [threshold, leaf(0)])
        );
    }

    #[test]
    fn sargon_shield_role() {
        // AnyOf[CountOf(2, [A, B, C]), AnyOf([A])]
        let threshold = at_least(2, [leaf(0), leaf(1), leaf(2)]);
        assert_eq!(
            at_least(1, [threshold.clone(), at_least(1, [leaf(0)])])
                .simplified(),
            at_least(1, [threshold, leaf(0)])
        );
        // AnyOf[CountOf(0, []), AnyOf([A])], with no threshold factors.
        assert_eq!(
            at_least(1, [at_least(0, []), at_least(1, [leaf(0)])]).simplified(),
            SUT::Anyone
        );
    }

    #[test]
    fn keeps_duplicates() {
        let twice = at_least(2, [leaf(0), leaf(0)]);
        assert_eq!(twice.clone().simplified(), twice);
    }

    /// Every requirement of depth one with up to three requirements, and of
    /// depth two with up to two requirements at each level.
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

    /// Random larger requirements, from a fixed seed so failures reproduce.
    #[test]
    fn keeps_meaning_of_random_requirements() {
        let auth_zones = auth_zones();
        let leaves = leaves();
        let mut random = Xorshift(0x5eed_acce_5500_0001);
        for _ in 0..20_000 {
            let requirement = random_requirement(&mut random, &leaves, 4);
            assert_simplifies_correctly(&requirement, &auth_zones);
        }
    }

    fn random_requirement(
        random: &mut Xorshift,
        leaves: &[SUT; 3],
        depth: u32,
    ) -> SUT {
        if depth == 0 || random.below(3) == 0 {
            return match random.below(5) {
                0 => SUT::Anyone,
                1 => SUT::NoOne,
                index => leaves[index as usize - 2].clone(),
            };
        }
        let len = random.below(5);
        let requirements = (0..len)
            .map(|_| random_requirement(random, leaves, depth - 1))
            .collect::<Vec<_>>();
        at_least(random.below(len + 2) as u32, requirements)
    }

    struct Xorshift(u64);

    impl Xorshift {
        fn below(&mut self, bound: u64) -> u64 {
            self.0 ^= self.0 << 13;
            self.0 ^= self.0 >> 7;
            self.0 ^= self.0 << 17;
            self.0 % bound
        }
    }
}
