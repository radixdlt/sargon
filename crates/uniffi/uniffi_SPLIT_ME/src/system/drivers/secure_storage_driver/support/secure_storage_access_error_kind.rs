use crate::prelude::*;
use sargon::SecureStorageAccessErrorKind as InternalSecureStorageAccessErrorKind;

/// An error kind that might be returned during access to secure storage driver. These errors are
/// android specific and are defined [here](https://developer.android.com/reference/android/hardware/biometrics/BiometricPrompt#constants_1)
/// Hosts, can print the error message provided by the system, and can ignore the error if
/// it `is_manual_cancellation`.
#[derive(Clone, PartialEq, InternalConversion, uniffi::Enum)]
pub enum SecureStorageAccessErrorKind {
    /// The hardware is unavailable. Try again later.
    HardwareUnavailable,

    /// The sensor was unable to process the current image.
    UnableToProcess,

    /// The current operation has been running too long and has timed out.
    // This is intended to prevent programs from waiting for the biometric sensor indefinitely.
    // The timeout is platform and sensor-specific, but is generally on the order of ~30 seconds.
    Timeout,

    /// The operation can't be completed because there is not enough device storage remaining.
    NoSpace,

    /// The operation was canceled because the biometric sensor is unavailable.
    /// This may happen when the user is switched, the device is locked, or another
    /// pending operation prevents it.
    Cancelled,

    /// The operation was canceled because the API is locked out due to too many attempts.
    /// This occurs after 5 failed attempts, and lasts for 30 seconds.
    Lockout,

    /// The operation failed due to a vendor-specific error.
    /// This error kind may be used by hardware vendors to extend this list to cover
    /// errors that don't fall under one of the other predefined categories. Vendors are
    /// responsible for providing the strings for these errors.
    ///
    /// These messages are typically reserved for internal operations such as enrollment
    /// but may be used to express any error that is not otherwise covered.
    /// In this case, applications are expected to show the error message, but they are advised
    /// not to rely on the message ID, since this may vary by vendor and device.
    Vendor,

    /// The operation was canceled because `Lockout` occurred too many times. Biometric
    /// authentication is disabled until the user unlocks with their device credential
    /// (i.e. PIN, pattern, or password).
    LockoutPermanent,

    /// The user canceled the operation.
    /// Upon receiving this, applications should use alternate authentication, such as a password.
    /// The application should also provide the user a way of returning to biometric authentication,
    /// such as a button.
    UserCancelled,

    /// The user does not have any biometrics enrolled.
    NoBiometrics,

    /// The device does not have the required authentication hardware.
    HardwareNotPresent,

    /// The user pressed the negative button.
    NegativeButton,

    /// The device does not have pin, pattern, or password set up.
    NoDeviceCredential,

    Unknown,
}

delegate_display_debug_into!(
    SecureStorageAccessErrorKind,
    InternalSecureStorageAccessErrorKind
);

#[uniffi::export]
pub fn secure_storage_access_error_kind_is_manual_cancellation(
    kind: SecureStorageAccessErrorKind,
) -> bool {
    kind.into_internal().is_manual_cancellation()
}

#[uniffi::export]
pub fn secure_storage_access_error_kind_to_string(
    kind: SecureStorageAccessErrorKind,
) -> String {
    kind.into_internal().to_string()
}
