use crate::prelude::*;

// use radix_engine_interface::blueprints::access_controller::{
//     AccessControllerTimedConfirmRecoveryInput as ScryptoAccessControllerTimedConfirmRecoveryInput,
//     ACCESS_CONTROLLER_TIMED_CONFIRM_RECOVERY_IDENT as SCRYPTO_ACCESS_CONTROLLER_TIMED_CONFIRM_RECOVERY_IDENT,
// };

pub trait TransactionManifestConfirmTimedRecovery {
    /// TBD
    /// TODO: Figure out how to do this...
    /// since `AccessControllerTimedConfirmRecoveryInput` need the input of
    /// the factors and time which was used to start recovery - which could not
    /// be quick confirmed. We need to figure out how we best represent this
    /// in Profile. Perhaps a new variant on ProvisionalSecurityConfig? Something
    /// like:
    /// `WaitingForTimedRecovery(SecurityStructureOfFactorInstances)`
    fn confirm_timed_recovery() -> Result<TransactionManifest>;
    // {
    //     builder.call_method(
    //          access_controller,
    //          SCRYPTO_ACCESS_CONTROLLER_TIMED_CONFIRM_RECOVERY_IDENT,
    //         ScryptoAccessControllerTimedConfirmRecoveryInput::from(
    //             factors_and_time,
    //         ),
    //     );
    // }
}
