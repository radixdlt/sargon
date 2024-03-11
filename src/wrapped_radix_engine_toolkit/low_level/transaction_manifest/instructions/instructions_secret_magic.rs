use crate::prelude::*;

/// An internal representation of a collection of Instructions,
/// which intentions is to allow the `struct Instructions`
/// to have no public initializers in Swift/Kotlin land, since it
/// can contain a field:
/// `private let secretMagic: InstructionsSecretMagic`
/// And hide its initializers.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct InstructionsSecretMagic(pub(crate) Vec<ScryptoInstruction>);

impl crate::UniffiCustomTypeConverter for InstructionsSecretMagic {
    type Builtin = BagOfBytes;

    fn into_custom(val: Self::Builtin) -> uniffi::Result<Self> {
        let bytes: &[u8] = val.bytes();
        RET_decompile_instructions(bytes)
            .map_err(|e| {
                let err_msg = format!("{:?}", e);
                error!("{}", err_msg);
                CommonError::FailedToUniFFIDecodeBytesToManifestInstructions
                    .into()
            })
            .map(|i: Vec<ScryptoInstruction>| Self(i))
    }

    fn from_custom(obj: Self) -> Self::Builtin {
        RET_compile_instructions(&obj.0)
            .map(|b| b.into())
            .expect("to never fail")
    }
}

impl From<ScryptoInstructions> for InstructionsSecretMagic {
    fn from(value: ScryptoInstructions) -> Self {
        Self(value.0)
    }
}

impl HasSampleValues for InstructionsSecretMagic {
    fn sample() -> Self {
        Self(vec![
            ScryptoInstruction::DropAuthZoneProofs, // sbor: 0x12
            ScryptoInstruction::DropAuthZoneRegularProofs, // sbor: 0x13
        ])
    }

    fn sample_other() -> Self {
        Self(vec![ScryptoInstruction::DropAuthZoneSignatureProofs]) // sbor: 0x17
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::prelude::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = InstructionsSecretMagic;

    #[test]
    fn equality() {
        assert_eq!(SUT::sample(), SUT::sample());
        assert_eq!(SUT::sample_other(), SUT::sample_other());
    }

    #[test]
    fn from_scrypto() {
        assert_eq!(
            SUT::sample(),
            ScryptoInstructions(vec![
                ScryptoInstruction::DropAuthZoneProofs,
                ScryptoInstruction::DropAuthZoneRegularProofs,
            ])
            .into()
        );
    }

    #[test]
    fn inequality() {
        assert_ne!(SUT::sample(), SUT::sample_other());
    }

    #[test]
    fn manual_perform_uniffi_conversion_successful() {
        let sut = SUT::sample();
        let builtin = BagOfBytes::from_hex("4d20220212001300").unwrap();

        let ffi_side =
            <SUT as crate::UniffiCustomTypeConverter>::from_custom(sut.clone());

        assert_eq!(ffi_side.to_hex(), builtin.to_hex());

        let from_ffi_side =
            <SUT as crate::UniffiCustomTypeConverter>::into_custom(ffi_side)
                .unwrap();

        assert_eq!(sut, from_ffi_side);
    }

    #[test]
    fn manual_perform_uniffi_conversion_fail() {
        let builtin = BagOfBytes::from_hex("deadbeef").unwrap();
        assert!(<SUT as crate::UniffiCustomTypeConverter>::into_custom(
            builtin
        )
        .is_err());
    }
}
