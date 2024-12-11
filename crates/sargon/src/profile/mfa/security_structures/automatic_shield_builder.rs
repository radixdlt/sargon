use std::future::Future;

use crate::prelude::*;

use super::security_shield_builder;

pub struct AutomaticShieldBuilder {
    remaining_available_factors: IndexSet<FactorSource>,
    picked_primary_role_factors: IndexSet<FactorSourceID>,
    shield_builder: SecurityShieldBuilder,
}

impl AutomaticShieldBuilder {
    fn find_primary_role_candidates(
        all: &IndexSet<FactorSource>,
        shield_builder: &SecurityShieldBuilder,
    ) -> IndexSet<FactorSource> {
        let factor_source_ids =
            all.iter().map(|f| f.id()).collect::<IndexSet<_>>();
        shield_builder.validation_for_addition_of_factor_source_to_primary_threshold_for_each(factor_source_ids.into_iter().collect_vec()).into_iter().filter(|vs| match vs.validation {
            Ok(_) => true,
            Err(RoleBuilderValidation::NotYetValid(_)) => true,
            Err(RoleBuilderValidation::BasicViolation(_)) |  Err(RoleBuilderValidation::ForeverInvalid(_)) => false,
        }).filter_map(|vs| all.iter().find(|f| f.id() == vs.factor_source_id))
        .cloned()
        .collect::<IndexSet<_>>()
    }

    fn factors_of_category(
        &self,
        category: FactorSourceCategory,
    ) -> IndexSet<FactorSource> {
        self.remaining_available_factors
            .iter()
            .filter(|f| f.category() == category)
            .sorted_by_key(|&f| f.common_properties().last_used_on)
            .cloned()
            .collect::<IndexSet<_>>()
    }

    fn consume(&mut self, factor: FactorSourceID) {
        self.remaining_available_factors
            .retain(|f| f.id() != factor);
    }

    fn consume_factor_and_add_to(
        &mut self,
        factor: FactorSourceID,
        add_to: &mut IndexSet<FactorSourceID>,
    ) {
        let was_inserted = add_to.insert(factor);
        assert!(was_inserted);
        self.consume(factor);
    }
}

struct ShouldAddFactorToListEvaluation {
    target_categories_with_kind_restrictions:
        IndexMap<FactorSourceCategory, IndexSet<FactorSourceKind>>,
    target_amount: Option<Amount>, // None means use ALL
}
impl ShouldAddFactorToListEvaluation {
    fn target_categories(&self) -> IndexSet<FactorSourceCategory> {
        self.target_categories_with_kind_restrictions
            .keys()
            .cloned()
            .collect()
    }

    fn with(
        target_categories_with_kind_restrictions: IndexMap<
            FactorSourceCategory,
            IndexSet<FactorSourceKind>,
        >,
        target_amount: impl Into<Option<Amount>>,
    ) -> Self {
        target_categories_with_kind_restrictions
            .iter()
            .for_each(|(k, v)| {
                v.iter().for_each(|x| assert_eq!(x.category(), *k))
            });

        Self {
            target_categories_with_kind_restrictions,
            target_amount: target_amount.into(),
        }
    }

    fn new(
        target_categories: impl IntoIterator<Item = FactorSourceCategory>,
        target_amount: impl Into<Option<Amount>>,
    ) -> Self {
        Self::with(
            target_categories
                .into_iter()
                .map(|c| (c, IndexSet::new()))
                .collect::<IndexMap<_, _>>(),
            target_amount,
        )
    }

    fn is_required(&self) -> bool {
        let Some(target_amount) = &self.target_amount else {
            return false;
        };
        target_amount.is_required
    }

    fn assert_fulfilled(&self, actual: usize) -> Result<()> {
        let Some(target_amount) = &self.target_amount else {
            return Ok(());
        };
        target_amount.assert_fulfilled(actual)
    }

    /// Nil means the requirement cannot be fulfilled by ADDING factors, since too
    /// many are already present.
    fn number_of_factors_of_category_to_add(
        &self,
        category: FactorSourceCategory,
        current_len_of_factor_list: usize,
    ) -> Option<usize> {
        let Some(target_amount) = &self.target_amount else {
            return Some(usize::MAX); // Add "All"
        };
        if self.target_categories().contains(&category) {
            target_amount.left_until_fulfilled(current_len_of_factor_list)
        } else {
            Some(0)
        }
    }
}

impl AutomaticShieldBuilder {
    fn add_quantified_factors_of_categories_to_set_if_able(
        &mut self,
        to: &mut IndexSet<FactorSourceID>,
        eval: ShouldAddFactorToListEvaluation,
    ) -> Result<()> {
        for category in eval.target_categories().iter() {
            let factors_of_category = self.factors_of_category(*category);

            let Some(q) =
                eval.number_of_factors_of_category_to_add(*category, to.len())
            else {
                continue;
            };

            if q == 0 {
                continue;
            }

            if factors_of_category.len() < q && eval.is_required() {
                return Err(CommonError::AutomaticShieldBuildingFailure {
                    underlying: format!(
                        "Not enough factors of category {:?}",
                        category
                    ),
                });
            }

            let quantified_factors = factors_of_category
                .iter()
                .take(q)
                .cloned()
                .collect::<IndexSet<_>>();

            quantified_factors.into_iter().for_each(|factor| {
                self.consume_factor_and_add_to(factor.id(), to);
            });
        }

        eval.assert_fulfilled(to.len())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Amount {
    is_required: bool,
    quantity: RequestedQuantity,
}

impl Amount {
    fn new(is_required: bool, quantity: RequestedQuantity) -> Self {
        Self {
            is_required,
            quantity,
        }
    }

    /// `None` means the requirement cannot be fulfilled by ADDING factors, since too
    /// many are already present.
    fn left_until_fulfilled(&self, actual: usize) -> Option<usize> {
        let left = self.quantity.left_until_fulfilled(actual);
        if left < 0 {
            None
        } else {
            Some(left as usize)
        }
    }

    fn is_fulfilled(&self, actual: usize) -> bool {
        if let Some(remaining) = self.left_until_fulfilled(actual) {
            remaining == 0
        } else {
            false
        }
    }

    fn assert_fulfilled(&self, actual: usize) -> Result<()> {
        if !self.is_required {
            return Ok(());
        }
        if !self.is_fulfilled(actual) {
            return Err(CommonError::AutomaticShieldBuildingFailure {
                underlying: format!(
                    "Quantity requirement not met: {:?} != {:?}",
                    self, actual
                ),
            });
        }
        Ok(())
    }
}

impl AutomaticShieldBuilder {
    fn add_one_hardware_factor_to_set_if_able(
        &mut self,
        to: &mut IndexSet<FactorSourceID>,
        target_amount: Amount,
    ) -> Result<()> {
        self.add_quantified_factors_of_categories_to_set_if_able(
            to,
            ShouldAddFactorToListEvaluation::new(
                [FactorSourceCategory::Hardware],
                target_amount,
            ),
        )
    }

    fn add_factors_to_role(
        &self,
        factors: &IndexSet<FactorSourceID>,
        role: RoleKind,
    ) {
        factors.into_iter().for_each(|&f| match role {
            RoleKind::Primary => {
                self.shield_builder
                    .add_factor_source_to_primary_threshold(f);
            }
            RoleKind::Recovery => {
                self.shield_builder
                    .add_factor_source_to_recovery_override(f);
            }
            RoleKind::Confirmation => {
                self.shield_builder
                    .add_factor_source_to_confirmation_override(f);
            }
        });
    }

    fn _build_shield(&mut self) -> Result<SecurityStructureOfFactorSourceIDs> {
        // Primary
        {
            if self.picked_primary_role_factors.len() == 1 {
                // if the user chose only 1 that factor cannot be used in the recovery or confirmation roles
                self.remaining_available_factors
                    .retain(|f| f.id() != self.picked_primary_role_factors[0]);
            }
            self.add_factors_to_role(
                &self.picked_primary_role_factors,
                RoleKind::Primary,
            );
            self.shield_builder
                .set_threshold(self.picked_primary_role_factors.len() as u8);
        }

        let (recovery_factors, confirmation_factors) = {
            let mut recovery_factors = self
                .factors_of_category(FactorSourceCategory::Contact)
                .iter()
                .map(|f| f.id())
                .collect::<IndexSet<_>>();

            let mut confirmation_factors = self
                .factors_of_category(FactorSourceCategory::Information)
                .iter()
                .map(|f| f.id())
                .collect::<IndexSet<_>>();

            self.add_one_hardware_factor_to_set_if_able(
                &mut recovery_factors,
                Amount::new(true, RequestedQuantity::at_least(1)),
            )?;

            self.add_quantified_factors_of_categories_to_set_if_able(
                &mut recovery_factors,
                ShouldAddFactorToListEvaluation::new(
                    [FactorSourceCategory::Custodian],
                    Amount::new(false, RequestedQuantity::at_least(1)),
                ),
            )?;

            self.add_one_hardware_factor_to_set_if_able(
                &mut confirmation_factors,
                Amount::new(true, RequestedQuantity::at_least(1)),
            )?;

            self.add_quantified_factors_of_categories_to_set_if_able(
                &mut confirmation_factors,
                ShouldAddFactorToListEvaluation::new(
                    [FactorSourceCategory::Custodian],
                    Amount::new(false, RequestedQuantity::at_least(1)),
                ),
            )?;

            // "Distribute to try to get up to 2 RECOVERY and then 2 CONFIRM factors if possible"
            {
                self.add_one_hardware_factor_to_set_if_able(
                    &mut recovery_factors,
                    Amount::new(false, RequestedQuantity::exactly(2)),
                )?;

                self.add_one_hardware_factor_to_set_if_able(
                    &mut confirmation_factors,
                    Amount::new(false, RequestedQuantity::exactly(2)),
                )?;
            }

            // "Add any (and all) remaining Hardware or Custodian factors in the list to RECOVERY."
            self.add_quantified_factors_of_categories_to_set_if_able(
                &mut recovery_factors,
                ShouldAddFactorToListEvaluation::new(
                    [
                        FactorSourceCategory::Hardware,
                        FactorSourceCategory::Custodian,
                    ],
                    None,
                ),
            )?;

            // "Set all Biometrics/PIN factors to a role (they must be all in one role because they
            // are unlocked by the same Biometrics/PIN check)":
            let target_list =
                if recovery_factors.len() > confirmation_factors.len() {
                    // "If there are more RECOVERY factors than CONFIRM factors, add any (and all) Biometrics/PIN factors to CONFIRM"
                    &mut confirmation_factors
                } else {
                    // "Else, add any (and all) Biometrics/PIN factors to RECOVERY."
                    &mut recovery_factors
                };

            self.add_quantified_factors_of_categories_to_set_if_able(
                target_list,
                ShouldAddFactorToListEvaluation::with(
                    IndexMap::kv(
                        FactorSourceCategory::Identity,
                        IndexSet::just(FactorSourceKind::Device),
                    ),
                    None,
                ),
            )?;

            Ok((recovery_factors, confirmation_factors))
        }?;

        self.add_factors_to_role(&recovery_factors, RoleKind::Recovery);
        self.add_factors_to_role(&confirmation_factors, RoleKind::Confirmation);

        self.shield_builder.build().map_err(|e| {
            CommonError::AutomaticShieldBuildingFailure {
                underlying: format!("{:?}", e),
            }
        })
    }

    fn build_shield(self) -> Result<SecurityStructureOfFactorSourceIDs> {
        let mut _self = self;
        _self._build_shield()
    }
}

impl SecurityShieldBuilder {
    /// Returns the status of the prerequisites for building a Security Shield.
    ///
    /// According to [definition][doc], a Security Shield can be built if the user has, asides from
    /// the Identity factor, "2 or more factors, one of which must be Hardware"
    ///
    /// [doc]: https://radixdlt.atlassian.net/wiki/spaces/AT/pages/3758063620/MFA+Rules+for+Factors+and+Security+Shields#Factor-Prerequisites
    pub fn prerequisites_status(
        factor_source_ids: &IndexSet<FactorSourceID>,
    ) -> SecurityShieldPrerequisitesStatus {
        let count_excluding_identity = factor_source_ids
            .iter()
            .filter(|f| f.category() != FactorSourceCategory::Identity)
            .count();
        let count_hardware = factor_source_ids
            .iter()
            .filter(|f| f.category() == FactorSourceCategory::Hardware)
            .count();
        if count_hardware < 1 {
            SecurityShieldPrerequisitesStatus::HardwareRequired
        } else if count_excluding_identity < 2 {
            SecurityShieldPrerequisitesStatus::AnyRequired
        } else {
            SecurityShieldPrerequisitesStatus::Sufficient
        }
    }
}

pub struct AutoShieldBuilderValidatorOfPickedPrimaryFactors {
    #[allow(dead_code)]
    #[doc(hidden)]
    hidden: HiddenConstructor,
}
impl AutoShieldBuilderValidatorOfPickedPrimaryFactors {
    fn new() -> Self {
        Self {
            hidden: HiddenConstructor,
        }
    }

    pub fn validate_picked(
        &self,
        picked: IndexSet<FactorSourceID>,
    ) -> Result<ValidatedPrimary> {
        let ephemeral = SecurityShieldBuilder::new();
        ephemeral.set_threshold(picked.len() as u8);
        picked.iter().for_each(|f| {
            ephemeral.add_factor_source_to_primary_threshold(*f);
        });
        if let Some(invalid_reason) = ephemeral.validate_primary_role() {
            Err(CommonError::AutomaticShieldBuildingFailure {
                underlying: format!(
                    "Invalid picked for primary: {:?}",
                    invalid_reason
                ),
            })
        } else {
            // valid!
            let valid = unsafe { ValidatedPrimary::new(picked) };
            Ok(valid)
        }
    }
}

pub struct ValidatedPrimary {
    validated_picked: IndexSet<FactorSourceID>,
}
impl ValidatedPrimary {
    /// # Safety
    /// Rust memory safe, but marked "unsafe" since it might allow for specification
    /// of unsafe - as in application **unsecure** - factors for PrimaryRole, which might
    /// lead to increase risk for end user to loose funds.
    pub unsafe fn new(validated_picked: IndexSet<FactorSourceID>) -> Self {
        Self { validated_picked }
    }

    pub fn validated_picked(&self) -> IndexSet<FactorSourceID> {
        self.validated_picked.clone()
    }
}

impl AutomaticShieldBuilder {
    pub async fn build<Fut>(
        all_factors: IndexSet<FactorSource>,
        pick_primary_role_factors: impl Fn(
            IndexSet<FactorSource>,
            AutoShieldBuilderValidatorOfPickedPrimaryFactors,
        ) -> Fut,
    ) -> Result<SecurityStructureOfFactorSourceIDs>
    where
        Fut: Future<Output = ValidatedPrimary>,
    {
        if !SecurityShieldBuilder::prerequisites_status(
            &all_factors.iter().map(|f| f.id()).collect(),
        )
        .is_sufficient()
        {
            return Err(CommonError::AutomaticShieldBuildingFailure {
                underlying: "Prerequisites not met".to_string(),
            });
        }
        let security_shield_builder = SecurityShieldBuilder::new();
        let candidates = Self::find_primary_role_candidates(
            &all_factors,
            &security_shield_builder,
        );
        let validated_picked: IndexSet<FactorSourceID> =
            pick_primary_role_factors(
                candidates,
                AutoShieldBuilderValidatorOfPickedPrimaryFactors::new(),
            )
            .await
            .validated_picked;

        let auto_builder = Self {
            remaining_available_factors: all_factors.into_iter().collect(),
            picked_primary_role_factors: validated_picked,
            shield_builder: security_shield_builder,
        };
        auto_builder.build_shield()
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Mutex;

    use async_std::future::ready;
    use indexmap::IndexSet;

    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = AutomaticShieldBuilder;

    impl SUT {
        #[allow(dead_code)]
        async fn test_non_validated<Fut>(
            pick_primary_role_factors: impl Fn(
                IndexSet<FactorSource>,
                AutoShieldBuilderValidatorOfPickedPrimaryFactors,
            ) -> Fut,
        ) -> Result<SecurityStructureOfFactorSourceIDs>
        where
            Fut: Future<Output = ValidatedPrimary>,
        {
            SUT::build(FactorSource::sample_all(), pick_primary_role_factors)
                .await
        }

        async fn test_valid(
            pick_primary_role_factors: impl Fn(
                IndexSet<FactorSource>,
            )
                -> IndexSet<FactorSourceID>,
        ) -> Result<SecurityStructureOfFactorSourceIDs> {
            SUT::build(
                FactorSource::sample_all(),
                async |candidates, validator| {
                    let picked = pick_primary_role_factors(candidates);
                    validator.validate_picked(picked).unwrap()
                },
            )
            .await
        }
    }

    #[actix_rt::test]
    async fn primary_role_candidates() {
        let shield_builder = SecurityShieldBuilder::new();

        let expected =  shield_builder.validation_for_addition_of_factor_source_to_primary_threshold_for_each(
                FactorSource::sample_all().into_iter().map(|f| f.id()).collect_vec()
            )
            .into_iter()
            .filter(|f| matches!(f.validation, Err(RoleBuilderValidation::NotYetValid(_))) || f.validation.is_ok())
            .map(|vs| vs.factor_source_id)
            .collect_vec();

        let called = Arc::new(Mutex::new(false));

        let _ = SUT::test_valid(|candidates| {
            *called.lock().unwrap() = true;
            pretty_assertions::assert_eq!(
                candidates.into_iter().map(|f| f.id()).collect_vec(),
                expected
            );

            IndexSet::just(FactorSourceID::sample_device())
        })
        .await
        .unwrap();

        assert!(*called.lock().unwrap());
    }

    #[actix_rt::test]
    async fn selection_of_primary_factor_first() {
        let built = SUT::test_valid(|xs| {
            IndexSet::just(xs.iter().map(|x| x.id()).next().unwrap())
        })
        .await
        .unwrap();
        assert_eq!(
            built.matrix_of_factors.primary_role.get_threshold_factors(),
            &vec![FactorSource::sample_all().first().unwrap().id()]
        );
    }

    #[actix_rt::test]
    async fn selection_of_primary_factor_last() {
        let built = SUT::test_valid(|xs| {
            IndexSet::just(xs.iter().map(|x| x.id()).last().unwrap())
        })
        .await
        .unwrap();
        assert_eq!(
            built.matrix_of_factors.primary_role.get_threshold_factors(),
            &vec![FactorSource::sample_all()
                .into_iter()
                .filter(|f| f.category() == FactorSourceCategory::Identity)
                .last()
                .unwrap()
                .id()]
        );
    }

    #[actix_rt::test]
    async fn selection_of_primary_factor_two() {
        let factors = IndexSet::from_iter([
            FactorSourceID::sample_ledger(),
            FactorSourceID::sample_device(),
        ]);
        let built = SUT::test_valid(|_| factors.clone()).await.unwrap();
        pretty_assertions::assert_eq!(
            built.matrix_of_factors.primary_role.get_threshold_factors(),
            &factors.into_iter().collect_vec()
        );
    }

    #[actix_rt::test]
    #[should_panic]
    async fn selection_of_primary_invalid_only_one_password() {
        let _ = SUT::test_valid(
            |_| IndexSet::just(FactorSourceID::sample_password()), // invalid for primary
        )
        .await;
    }

    #[actix_rt::test]
    async fn empty_factors_is_err() {
        let called = Arc::new(Mutex::new(false));

        let res = SUT::build(IndexSet::new(), |_, _| {
            *called.lock().unwrap() = true;
            ready(unsafe {
                ValidatedPrimary::new(IndexSet::just(
                    FactorSourceID::sample_device(),
                ))
            })
        })
        .await;

        assert!(!*called.lock().unwrap());

        assert!(matches!(
            res,
            Err(CommonError::AutomaticShieldBuildingFailure { .. })
        ));
    }

    #[actix_rt::test]
    async fn one_factors_is_not_enough_is_err() {
        let called = Arc::new(Mutex::new(false));

        let res = SUT::build(
            IndexSet::from_iter([FactorSource::sample_device()]),
            |_, _| {
                *called.lock().unwrap() = true;
                ready(unsafe {
                    ValidatedPrimary::new(IndexSet::just(
                        FactorSourceID::sample_device(),
                    ))
                })
            },
        )
        .await;

        assert!(!*called.lock().unwrap());

        assert!(matches!(
            res,
            Err(CommonError::AutomaticShieldBuildingFailure { .. })
        ));
    }

    #[actix_rt::test]
    async fn two_factors_is_not_enough_is_err() {
        let called = Arc::new(Mutex::new(false));

        let res = SUT::build(
            IndexSet::from_iter([
                FactorSource::sample_device(),
                FactorSource::sample_ledger(),
            ]),
            |_, _| {
                *called.lock().unwrap() = true;
                ready(unsafe {
                    ValidatedPrimary::new(IndexSet::just(
                        FactorSourceID::sample_device(),
                    ))
                })
            },
        )
        .await;

        assert!(!*called.lock().unwrap());

        assert!(matches!(
            res,
            Err(CommonError::AutomaticShieldBuildingFailure { .. })
        ));
    }

    #[actix_rt::test]
    async fn two_device_factor_source_and_one_ledger_is_not_sufficient() {
        let called = Arc::new(Mutex::new(false));

        let res = SUT::build(
            IndexSet::from_iter([
                FactorSource::sample_device_babylon(),
                FactorSource::sample_device_babylon_other(),
                FactorSource::sample_ledger(),
            ]),
            |_, _| {
                *called.lock().unwrap() = true;
                ready(unsafe {
                    ValidatedPrimary::new(IndexSet::just(
                        FactorSourceID::sample_device(),
                    ))
                })
            },
        )
        .await;

        assert!(!*called.lock().unwrap());

        assert!(matches!(
            res,
            Err(CommonError::AutomaticShieldBuildingFailure { .. })
        ));
    }

    #[actix_rt::test]
    async fn one_device_factor_source_and_two_ledger_is_not_ok_if_ledger_is_used_for_primary(
    ) {
        let res = SUT::build(
            IndexSet::from_iter([
                FactorSource::sample_device_babylon(),
                FactorSource::sample_ledger(),
                FactorSource::sample_ledger_other(),
            ]),
            |_, _| {
                ready(unsafe {
                    ValidatedPrimary::new(IndexSet::just(
                        FactorSource::sample_ledger().id(),
                    ))
                })
            },
        )
        .await;

        println!("🔮 {:?}", res);
        assert!(matches!(
            res,
            Err(CommonError::AutomaticShieldBuildingFailure { .. })
        ));
    }
}

/*
enum Quantity {
    All,
    Fixed(u8)
    pub fn one() -> Self {
        Self::Fixed(1)
    }
}

enum FactorSelector {
    Category(FactorCategory),
    Kind(FactorSourceKind)
}

pub struct AutoShieldBuilder {
    shield_builder: SecurityShieldBuilder,
    all_available_factors: IndexSet<FactorSource>
}

impl AutoShieldBuilder {

    pub fn new(all_available_factors: IndexSet<FactorSource>, user_chosen) {
        let candiates_for_primary_role = filter_out_valid_primary_role_factors_from(all_available_factors);
        let user_chosen_primary_factors = wallet_ui.user_selects(candiates_for_primary_role).await;

        // 📒 "If the user only chose 1 factor for ACCESS, remove that factor from the list (it cannot be used elsewhere - otherwise it can)."
        if user_chosen_primary_factors.len() == 1 {
            all_available_factors.remove_one(user_chosen_primary_factors.first())
        }

        shield_builder.set_threshold(user_chosen_primary_factors.len());
    }

    fn assign_factors_matching_selector(
        &mut self,
        to: &mut IndexSet<FactorSource>,
        selector: FactorSelector,
        quantity_to_add: Quantity
    ) {
        let all_filtered = self.all_available_factors.filter(selector)
        let quantified_filtered = all_filtered.take(quantity);
        target_factors.add_all(quantified_filtered)
        self.all_available_factors.remove_all(quantified_filtered)
    }


    fn assign_factors_of_category(
        &mut self,
        to: &mut IndexSet<FactorSource>,
        category: FactorCategory,
        quantity_to_add: Quantity
    ) {
        self.assign_factors_matching_selector(
            to: to,
            selector: FactorSelector::Category(category),
            quantity_to_add: quantity_to_add
        )
    }

    fn assign_factors_of_kind(
        &mut self,
        to: &mut IndexSet<FactorSource>,
        kind: FactorSourceKind,
        quantity_to_add: Quantity
    ) {
        self.assign_factors_matching_selector(
            to: to,
            selector: FactorSelector::Kind(kind),
            quantity_to_add: quantity_to_add
        )
    }


    fn assign_one_hardware_factor(
        &mut self,
        to: &mut IndexSet<FactorSource>,
    ) {
        self.assign_factors_of_category(
            to: to,
            category: FactorCategory::Hardware,
            quantity_to_add: Quantity::one()
        )
    }

    fn assign_factors_of_category_to_primary(
        &mut self,
        category: FactorCategory,
        quantity_to_add: Quantity
    ) {
        self.assign_factors_of_category(
            to: primary,
            category: category,
            quantity_to_add: quantity_to_add
        )
        // For Primary role, dont forget to update threshold
        shield_builder.set_threshold(primary.len());
    }


    fn assign_factors_of_category_to_recovery(
        &mut self,
        category: FactorCategory,
        quantity_to_add: Quantity
    ) {
        self.assign_factors_of_category(
            to: recovery,
            category: category,
            quantity_to_add: quantity_to_add
        )
    }


    fn assign_factors_of_category_to_confirmation(
        &mut self,
        category: FactorCategory,
        quantity_to_add: Quantity
    ) {
        self.assign_factors_of_category(
            to: confirmation,
            category: category,
            quantity_to_add: quantity_to_add
        )
    }

    pub fn auto_build_factors(&mut self) -> MatrixOfFactorSources {

        let mut primary = self.user_chosen_primary_factors;
        // Non user chosen factors, auto chosen by the heuristics laid out by Matt
        // in: https://radixdlt.atlassian.net/wiki/spaces/AT/pages/3758063620/MFA+Rules+for+Factors+and+Security+Shields
        let mut recovery: IndexSet<FactorSource>;
        let mut confirmation: IndexSet<FactorSource>;


        // 📒 "Drop in the somewhat “special-use” factors first"
        {
            // 📒	"Add all Contact factors in the list to START."
            self.assign_factors_of_category_to_recovery(
                category: FactorCategory::Contact,
                quantity_to_add: Quantity::All
            )

            // 📒	"Add all Information factors in the list to CONFIRM."
            self.assign_factors_of_category_to_confirmation(
                category: FactorCategory::Information,
                quantity_to_add: Quantity::All
            )
        }


        // 📒 "Distribute to try to get at least 1 START and then 1 CONFIRM"
        {
            // 📒 "Add any Custodian.." 🙅‍♀️  Custodian FactorSources does not exist yet...

            // 📒 "Add any Hardware factors in the list, starting with the most recently used, to START
            // until there is at least 1 factor source in START."
            self.assign_one_hardware_factor(
                to: recovery
            )
            // ❓ WHAT TO DO IF THERE WAS NONE?


            // 📒 "Add any Hardware (Ledger, Arculus, Yubikey) factors in the list, starting with the most recently used, to CONFIRM until there is at least 1 factor factors in CONFIRM."
            self.assign_one_hardware_factor(
                to: confirmation,
            )
            // ❓ WHAT TO DO IF THERE WAS NONE?
        }



        // 📒 "Distribute to try to get up to 2 START and then 2 CONFIRM factors if possible"
        {
            // 📒 "Add any Custodian.." 🙅‍♀️ Custodian FactorSources does not exist yet...

            // 📒 "Add any Hardware (Ledger, Arculus, Yubikey) factors in the list, starting with the most recently used, to START until there are exactly 2 factors in START."
            self.assign_one_hardware_factor(
                to: recovery
            )
            // ❓ WHAT TO DO IF THERE WAS NONE?


            // 📒 "Add any Hardware (Ledger, Arculus, Yubikey) factors in the list, starting with the most recently used, to CONFIRM until there are exactly 2 factors in CONFIRM."
            self.assign_one_hardware_factor(
                to: confirmation,
            )
            // ❓ WHAT TO DO IF THERE WAS NONE?
        }

        // 📒 "Fill in any remaining other factors to increase reliability of being able to recover"
        {
            // 📒 "Add any (and all) remaining Hardware or Custodian factors in the list to START."
            self.assign_factors_of_category_to_primary(
                category: FactorSourceKind::Hardware,
                quantity: Quantity::All
            )

            // 📒 "Set all Biometrics/PIN factors to a role (they must be all in one role because they are unlocked by the same Biometrics/PIN check):"

            let mut target_remaining_device_factors =  if recovery.len() > confirmation.len() {
                // 📒 "If there are more START factors than CONFIRM factors, add any (and all) Biometrics/PIN factors to CONFIRM"
                &mut confirmation
            } else {
                // 📒 "Else, add any (and all) Biometrics/PIN factors to START."
                &mut recovery
            };

            self.assign_factors_of_kind(
                to: target_remaining_device_factors,
                kind: FactorSourceKind::Device,
                quantity_to_add: Quantity::All
            )
        }
    }
}
*/
