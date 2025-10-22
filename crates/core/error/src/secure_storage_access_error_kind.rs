/// An error kind that might be returned during access to secure storage driver. These errors are
/// android specific and are defined [here](https://developer.android.com/reference/android/hardware/biometrics/BiometricPrompt#constants_1)
/// Hosts, can print the error message provided by the system, and can ignore the error if
/// it `is_manual_cancellation`.
#[derive(
    Clone, Debug, Eq, derive_more::Display, PartialEq, strum::EnumIter,
)]
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

impl SecureStorageAccessErrorKind {
    pub fn is_manual_cancellation(&self) -> bool {
        self == &SecureStorageAccessErrorKind::UserCancelled
            || self == &SecureStorageAccessErrorKind::NegativeButton
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::*;

    #[test]
    fn user_cancelled_is_manual() {
        let sut = SecureStorageAccessErrorKind::UserCancelled;
        assert!(sut.is_manual_cancellation())
    }

    #[test]
    fn negative_button_is_manual() {
        let sut = SecureStorageAccessErrorKind::NegativeButton;
        assert!(sut.is_manual_cancellation())
    }

    #[test]
    fn rest_are_system_errors() {
        assert!(!SecureStorageAccessErrorKind::HardwareUnavailable
            .is_manual_cancellation());
        assert!(!SecureStorageAccessErrorKind::UnableToProcess
            .is_manual_cancellation());
        assert!(!SecureStorageAccessErrorKind::Timeout.is_manual_cancellation());
        assert!(!SecureStorageAccessErrorKind::NoSpace.is_manual_cancellation());
        assert!(
            !SecureStorageAccessErrorKind::Cancelled.is_manual_cancellation()
        );
        assert!(!SecureStorageAccessErrorKind::Lockout.is_manual_cancellation());
        assert!(!SecureStorageAccessErrorKind::Vendor.is_manual_cancellation());
        assert!(!SecureStorageAccessErrorKind::LockoutPermanent
            .is_manual_cancellation());
        assert!(!SecureStorageAccessErrorKind::NoBiometrics
            .is_manual_cancellation());
        assert!(!SecureStorageAccessErrorKind::HardwareNotPresent
            .is_manual_cancellation());
        assert!(!SecureStorageAccessErrorKind::NoDeviceCredential
            .is_manual_cancellation());
    }
}
