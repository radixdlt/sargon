// pub enum DetailedManifestClass {
//     General,
//     Transfer,

//     ValidatorClaim {
//         validatorAddresses: [ValidatorAddress],
// validatorClaims: [TrackedValidatorClaim])

//     },

//     ValidatorStake {
//         validator_addresses: Vec<ValidatorAddress>,
//         validator_stakes: Vec<TrackedValidatorStake>,
//     },

//     ValidatorUnstake {
//         validator_addresses: Vec<ValidatorAddress>,
//         validator_unstakes: Vec<TrackedValidatorUnstake>,
//         claims_non_fungible_data: Vec<UnstakeDataEntry>,
//     },

//     AccountDepositSettingsUpdate {
//         resource_preferences_updates: HashMap<
//             AccountAddress,
//             HashMap<ResourceAddress, ResourcePreferenceUpdate>,
//         >,
//         deposit_mode_updates:
//             HashMap<AccountAddress, AccountDefaultDepositRule>,
//         authorized_depositors_added:
//             HashMap<AccountAddress, Vec<ResourceOrNonFungible>>,
//         authorized_depositors_removed:
//             HashMap<AccountAddress, Vec<ResourceOrNonFungible>>,
//     },

//     PoolContribution {
//         poolAddresses: Vec<ComponentAddress>,
//         poolContributions: Vec<TrackedPoolContribution>,
//     },

//     PoolRedemption {
//         poolAddresses: Vec<ComponentAddress>,
//         poolContributions: Vec<TrackedPoolRedemption>,
//     },
// }
