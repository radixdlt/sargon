use crate::prelude::*;

json_string_convertible!(SLIP10Curve, "super invalid json string");

#[uniffi::export]
pub fn new_slip10_curve_from_string(curve: String) -> Result<SLIP10Curve> {
    SLIP10Curve::from_str(&curve)
}

#[uniffi::export]
pub fn slip10_curve_to_string(curve: SLIP10Curve) -> String {
    curve.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_string_roundtrip() {
        assert_eq!(
            new_slip10_curve_from_string(slip10_curve_to_string(
                SLIP10Curve::Curve25519
            ))
            .unwrap(),
            SLIP10Curve::Curve25519
        )
    }
}
