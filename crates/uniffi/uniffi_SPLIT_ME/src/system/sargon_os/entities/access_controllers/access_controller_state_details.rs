use crate::prelude::*;
use sargon::AccessControllerStateDetails as InternalAccessControllerStateDetails;

#[derive(Clone, PartialEq, Eq, uniffi::Record)]
pub struct AccessControllerStateDetails {
    pub address: AccessControllerAddress,
    pub timed_recovery_state: Option<TimedRecoveryState>,
    pub xrd_balance: Decimal192,
}

/// Timed recovery state containing seconds until recovery confirmation will be possible.
#[derive(Clone, PartialEq, Eq, uniffi::Record)]
pub struct TimedRecoveryState {
    pub allow_timed_recovery_after_unix_timestamp_seconds: String,
}

impl From<InternalAccessControllerStateDetails>
    for AccessControllerStateDetails
{
    fn from(value: InternalAccessControllerStateDetails) -> Self {
        Self {
            address: value.address.into(),
            timed_recovery_state: value
                .state
                .recovery_role_recovery_attempt
                .and_then(|recovery_attempt| {
                    recovery_attempt
                        .allow_timed_recovery_after
                        .map(|allow_after| allow_after.unix_timestamp_seconds)
                })
                // .flatten()
                .map(|allow_after_seconds| TimedRecoveryState {
                    allow_timed_recovery_after_unix_timestamp_seconds:
                        allow_after_seconds,
                }),
            xrd_balance: value.xrd_balance.into(),
        }
    }
}
