use crate::prelude::*;
use std::cell::RefCell;

// #[derive(Clone, Debug, PartialEq, Eq)]
// pub struct SigningFactorList {
//     /// Will be `1` of override factors.
//     pub threshold: u8,
//     pub left_factors: RefCell<IndexSet<FactorInstance>>,
//     pub skipped_factors: RefCell<IndexSet<FactorInstance>>,

//     pub signed_factors: RefCell<IndexMap<FactorInstance, SignatureWithPublicKey>>,
// }
// impl SigningFactorList {
//     fn new(threshold: u8, factors: impl IntoIterator<Item = FactorInstance>) -> Self {
//         let left_factors = factors.into_iter().collect::<IndexSet<_>>();
//         Self {
//             left_factors: RefCell::new(left_factors),
//             threshold,
//             skipped_factors: RefCell::new(IndexSet::new()),
//             signed_factors: RefCell::new(IndexMap::new()),
//         }
//     }
//     fn single(instance: FactorInstance) -> Self {
//         Self::new(1, [instance])
//     }
// }

// #[derive(Clone, Debug, PartialEq, Eq)]
// pub struct SigningFactorMatrix {
//     pub threshold_factors: RefCell<SigningFactorList>,
//     pub override_factors: RefCell<SigningFactorList>,
// }
// impl SigningFactorMatrix {
//     fn new(threshold_factors: SigningFactorList, override_factors: SigningFactorList) -> Self {
//         Self {
//             threshold_factors: RefCell::new(threshold_factors),
//             override_factors: RefCell::new(override_factors)
//         }
//     }
//     fn new_unsecured(factor_instance: FactorInstance) -> Self {
//         Self::new(
//             SigningFactorList::single(factor_instance),

//         )
//     }
// }

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SigningFactors {
    // pub signatures: RefCell<IndexMap<FactorSourceKind, IndexMap<FactorSource, SigningFactorMatrix>>>
}

impl Profile {
    pub fn signing_factors_of(
        &self,
        entities: IndexSet<AccountOrPersona>,
    ) -> Result<SigningFactors> {
        // let mut set = IndexSet::<SigningFactorMatrix>::new();

        // entities.into_iter().map(|e|
        //     match e.security_state() {
        //         EntitySecurityState::Unsecured { value: unsecured_entity_control } => {
        //             set.insert(SigningFactorMatrix::new_unsecured(unsecured_entity_control.transaction_signing.factor_instance()))
        //         },
        //         EntitySecurityState::Secured { value: secured_entity_control } => {
        //             secured_entity_control
        //         },
        //     }

        // )
        todo!()
    }
}
