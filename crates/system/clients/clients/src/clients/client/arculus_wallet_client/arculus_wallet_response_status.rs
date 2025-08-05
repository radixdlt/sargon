use crate::prelude::*;

/*
#define CSDK_OK                        0        /**< Success return code */
/**************************************************************************************

 ERRORS DEFINITION

 ***************************************************************************************/
#define CSDK_ERR_NULL_POINTER               -100     /** Null pointer encountered */
#define CSDK_ERR_NULL_APPLETOBJ             -101     /** Wallet session object is NULL */
#define CSDK_ERR_NULL_CALLOC                -102     /** Unable to allocate memory */
#define CSDK_ERR_WRONG_RESPONSE_LENGTH      -103     /** Card response length is incorrect/unexpected */
#define CSDK_ERR_WRONG_RESPONSE_DATA        -104     /** Card response not valid */
#define CSDK_ERR_WRONG_STATUS_WORD          -105     /** Card response status not expected */
#define CSDK_ERR_WRONG_DATA_LENGTH          -106     /** Data length of payload is invalid */
#define CSDK_ERR_WRONG_PARAM_LENGTH         -107     /** Parameter size validation failed */
#define CSDK_ERR_WRONG_PIN                  -108     /** Wrong PIN */
#define CSDK_ERR_INVALID_PARAM              -109     /** Invalid Parameter */
#define CSDK_ERR_ENCRYPTION_NOT_INIT        -110     /** NFC Session encryption was not initialized */
#define CSDK_ERR_EXT_OR_CHAIN_NOT_SUPPORTED  -111     /** Card doesn't support extended APDUs or chaining */
#define CSDK_ERR_API_CHAIN_NOT_SUPPORTED     -112     /** API is deprecated and requires Chaining */
#define CSDK_ERR_UNKNOWN_ERROR              -113     /** An unknown error has occurred */
#define CSDK_ERR_APDU_EXCEEDS_CHAIN_LENGTH  -114     /** APDU too big to do chaining */
#define CSDK_ERR_EXTAPDU_SUPPORT_REQUIRED   -115     /** Extended APDU not supported but required */
#define CSDK_ERR_APDU_TOO_BIG               -116     /** APDU too big */
#define CSDK_ERR_WALLET_NOT_SELECTED        -117     /** Wallet not selected */
*/

/// The status of a response from the Arculus wallet.
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[repr(i32)]
pub enum ArculusWalletCSDKResponseStatus {
    /// Success return code
    Ok = 0,

    /// Null pointer encountered
    NullPointer = -100,

    /// Wallet session object is NULL
    NullWalletSession = -101,

    /// Unable to allocate memory
    NullCalloc = -102,

    /// Card response length is incorrect/unexpected
    WrongResponseLength = -103,

    /// Card response not valid
    WrongResponseData = -104,

    /// Card response status not expected
    WrongStatusWord = -105,

    /// Data length of payload is invalid
    WrongDataLength = -106,

    /// Parameter size validation failed
    WrongParamLength = -107,

    /// Wrong PIN
    WrongPin = -108,

    /// Invalid Parameter
    InvalidParam = -109,

    /// NFC Session encryption was not initialized
    EncryptionNotInit = -110,

    /// Card doesn't support extended APDUs or chaining
    ExtOrChainNotSupported = -111,

    /// API is deprecated and requires Chaining
    ApiChainNotSupported = -112,

    /// An unknown error has occurred
    UnknownError = -113,

    /// APDU too big to do chaining
    ApduExceedsChainLength = -114,

    /// Extended APDU not supported but required
    ExtApduSupportRequired = -115,

    /// APDU too big
    ApduTooBig = -116,

    /// Wallet not selected
    WalletNotSelected = -117,
}

impl TryFrom<i32> for ArculusWalletCSDKResponseStatus {
    type Error = CommonError;

    fn try_from(value: i32) -> Result<Self> {
        match value {
            0 => Ok(Self::Ok),
            -100 => Ok(Self::NullPointer),
            -101 => Ok(Self::NullWalletSession),
            -102 => Ok(Self::NullCalloc),
            -103 => Ok(Self::WrongResponseLength),
            -104 => Ok(Self::WrongResponseData),
            -105 => Ok(Self::WrongStatusWord),
            -106 => Ok(Self::WrongDataLength),
            -107 => Ok(Self::WrongParamLength),
            -108 => Ok(Self::WrongPin),
            -109 => Ok(Self::InvalidParam),
            -110 => Ok(Self::EncryptionNotInit),
            -111 => Ok(Self::ExtOrChainNotSupported),
            -112 => Ok(Self::ApiChainNotSupported),
            -113 => Ok(Self::UnknownError),
            -114 => Ok(Self::ApduExceedsChainLength),
            -115 => Ok(Self::ExtApduSupportRequired),
            -116 => Ok(Self::ApduTooBig),
            -117 => Ok(Self::WalletNotSelected),
            _ => Err(CommonError::ArculusCSDKUnknownResponseStatusCode {
                status_code: value,
            }),
        }
    }
}

impl ArculusWalletCSDKResponseStatus {
    pub fn as_result(self) -> Result<()> {
        match self {
            Self::Ok => Ok(()),
            _ => Err(CommonError::ArculusCSDKBadStatusCode {
                status_code: self as i32,
            }),
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn try_from_i32() {
        use super::*;

        assert_eq!(
            ArculusWalletCSDKResponseStatus::try_from(0),
            Ok(ArculusWalletCSDKResponseStatus::Ok)
        );
        assert_eq!(
            ArculusWalletCSDKResponseStatus::try_from(-100),
            Ok(ArculusWalletCSDKResponseStatus::NullPointer)
        );
        assert_eq!(
            ArculusWalletCSDKResponseStatus::try_from(-101),
            Ok(ArculusWalletCSDKResponseStatus::NullWalletSession)
        );
        assert_eq!(
            ArculusWalletCSDKResponseStatus::try_from(-102),
            Ok(ArculusWalletCSDKResponseStatus::NullCalloc)
        );
        assert_eq!(
            ArculusWalletCSDKResponseStatus::try_from(-103),
            Ok(ArculusWalletCSDKResponseStatus::WrongResponseLength)
        );
        assert_eq!(
            ArculusWalletCSDKResponseStatus::try_from(-104),
            Ok(ArculusWalletCSDKResponseStatus::WrongResponseData)
        );
        assert_eq!(
            ArculusWalletCSDKResponseStatus::try_from(-105),
            Ok(ArculusWalletCSDKResponseStatus::WrongStatusWord)
        );
        assert_eq!(
            ArculusWalletCSDKResponseStatus::try_from(-106),
            Ok(ArculusWalletCSDKResponseStatus::WrongDataLength)
        );
        assert_eq!(
            ArculusWalletCSDKResponseStatus::try_from(-107),
            Ok(ArculusWalletCSDKResponseStatus::WrongParamLength)
        );
        assert_eq!(
            ArculusWalletCSDKResponseStatus::try_from(-108),
            Ok(ArculusWalletCSDKResponseStatus::WrongPin)
        );
        assert_eq!(
            ArculusWalletCSDKResponseStatus::try_from(-109),
            Ok(ArculusWalletCSDKResponseStatus::InvalidParam)
        );
        assert_eq!(
            ArculusWalletCSDKResponseStatus::try_from(-110),
            Ok(ArculusWalletCSDKResponseStatus::EncryptionNotInit)
        );
        assert_eq!(
            ArculusWalletCSDKResponseStatus::try_from(-111),
            Ok(ArculusWalletCSDKResponseStatus::ExtOrChainNotSupported)
        );
        assert_eq!(
            ArculusWalletCSDKResponseStatus::try_from(-112),
            Ok(ArculusWalletCSDKResponseStatus::ApiChainNotSupported)
        );
        assert_eq!(
            ArculusWalletCSDKResponseStatus::try_from(-113),
            Ok(ArculusWalletCSDKResponseStatus::UnknownError)
        );
        assert_eq!(
            ArculusWalletCSDKResponseStatus::try_from(-114),
            Ok(ArculusWalletCSDKResponseStatus::ApduExceedsChainLength)
        );
        assert_eq!(
            ArculusWalletCSDKResponseStatus::try_from(-115),
            Ok(ArculusWalletCSDKResponseStatus::ExtApduSupportRequired)
        );
        assert_eq!(
            ArculusWalletCSDKResponseStatus::try_from(-116),
            Ok(ArculusWalletCSDKResponseStatus::ApduTooBig)
        );
        assert_eq!(
            ArculusWalletCSDKResponseStatus::try_from(-117),
            Ok(ArculusWalletCSDKResponseStatus::WalletNotSelected)
        );
        assert_eq!(
            ArculusWalletCSDKResponseStatus::try_from(-118),
            Err(CommonError::ArculusCSDKUnknownResponseStatusCode {
                status_code: -118
            })
        );
    }

    #[test]
    fn as_result() {
        use super::*;

        assert_eq!(ArculusWalletCSDKResponseStatus::Ok.as_result(), Ok(()));
        assert_eq!(
            ArculusWalletCSDKResponseStatus::NullPointer.as_result(),
            Err(CommonError::ArculusCSDKBadStatusCode { status_code: -100 })
        );
    }
}
