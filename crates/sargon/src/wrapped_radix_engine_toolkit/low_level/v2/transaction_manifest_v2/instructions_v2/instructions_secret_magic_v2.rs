use crate::prelude::*;
use radix_transactions::manifest::{
    DropAuthZoneProofs, DropAuthZoneRegularProofs, DropAuthZoneSignatureProofs,
};

/// An internal representation of a collection of Instructions,
/// which intentions is to allow the `struct InstructionsV2`
/// to have no public initializers in Swift/Kotlin land, since it
/// can contain a field:
/// `private let secretMagic: InstructionsSecretMagicV2`
/// And hide its initializers.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct InstructionsSecretMagicV2(pub Vec<ScryptoInstructionV2>);

impl InstructionsSecretMagicV2 {
    pub(crate) fn instructions(&self) -> &Vec<ScryptoInstructionV2> {
        &self.0
    }
    pub(crate) fn new(instructions: Vec<ScryptoInstructionV2>) -> Self {
        Self(instructions)
    }
}

uniffi::custom_type!(InstructionsSecretMagicV2, BagOfBytes);

impl crate::UniffiCustomTypeConverter for InstructionsSecretMagicV2 {
    type Builtin = BagOfBytes;

    fn into_custom(val: Self::Builtin) -> uniffi::Result<Self> {
        let bytes: &[u8] = val.bytes();
        RET_from_payload_bytes_instructions_v2(bytes)
            .map_err(|e| {
                let err_msg = format!("{:?}", e);
                error!("{}", err_msg);
                CommonError::FailedToUniFFIDecodeBytesToManifestInstructions
                    .into()
            })
            .map(Self::new)
    }

    fn from_custom(obj: Self) -> Self::Builtin {
        RET_to_payload_bytes_instructions_v2(&obj.0)
            .map(|b| b.into())
            .expect("to never fail")
    }
}

impl From<ScryptoInstructionsV2> for InstructionsSecretMagicV2 {
    fn from(value: ScryptoInstructionsV2) -> Self {
        Self(value.0.to_vec())
    }
}

impl HasSampleValues for InstructionsSecretMagicV2 {
    fn sample() -> Self {
        Self(vec![
            ScryptoInstructionV2::DropAuthZoneProofs(DropAuthZoneProofs), // sbor: 0x12
            ScryptoInstructionV2::DropAuthZoneRegularProofs(
                DropAuthZoneRegularProofs,
            ), // sbor: 0x13
        ])
    }

    fn sample_other() -> Self {
        Self(vec![ScryptoInstructionV2::DropAuthZoneSignatureProofs(
            DropAuthZoneSignatureProofs,
        )]) // sbor: 0x17
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::prelude::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = InstructionsSecretMagicV2;

    #[test]
    fn equality() {
        assert_eq!(SUT::sample(), SUT::sample());
        assert_eq!(SUT::sample_other(), SUT::sample_other());
    }

    #[test]
    fn from_scrypto() {
        assert_eq!(
            SUT::sample(),
            ScryptoInstructionsV2(
                vec![
                    ScryptoInstructionV2::DropAuthZoneProofs(
                        DropAuthZoneProofs
                    ),
                    ScryptoInstructionV2::DropAuthZoneRegularProofs(
                        DropAuthZoneRegularProofs
                    ),
                ]
                .into()
            )
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
