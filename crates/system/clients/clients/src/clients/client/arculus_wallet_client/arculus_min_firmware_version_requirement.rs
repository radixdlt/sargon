use crate::prelude::*;

#[derive(Clone, Debug, PartialEq)]
pub enum ArculusMinFirmwareVersionRequirement {
    Valid,
    Invalid(String),
}

impl ArculusMinFirmwareVersionRequirement {
    pub fn new(version: String) -> Self {
        // minimum acceptable firmware: 2.2.7.6
        const MIN_VERSION: [u32; 4] = [2, 2, 7, 6];

        // parse up to four dot-separated segments, missing ones stay 0
        let mut parts = [0u32; 4];
        for (i, seg) in version.split('.').take(4).enumerate() {
            parts[i] = seg.parse().unwrap_or(0);
        }

        // lexicographically compare; anything below is invalid
        if parts < MIN_VERSION {
            Self::Invalid(version)
        } else {
            Self::Valid
        }
    }
}

impl HasSampleValues for ArculusMinFirmwareVersionRequirement {
    fn sample() -> Self {
        Self::Valid
    }

    fn sample_other() -> Self {
        Self::Invalid("2.2.7.4".to_owned())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_below_min_version() {
        let v = "2.2.7.5".to_string();
        assert_eq!(
            ArculusMinFirmwareVersionRequirement::new(v.clone()),
            ArculusMinFirmwareVersionRequirement::Invalid(v)
        );
    }

    #[test]
    fn test_equal_min_version() {
        let v = "2.2.7.6".to_string();
        assert_eq!(
            ArculusMinFirmwareVersionRequirement::new(v),
            ArculusMinFirmwareVersionRequirement::Valid
        );
    }

    #[test]
    fn test_above_min_version() {
        for &s in &["2.2.7.7", "2.2.8.0", "2.3.0.0", "3.0.0.0"] {
            assert_eq!(
                ArculusMinFirmwareVersionRequirement::new(s.to_string()),
                ArculusMinFirmwareVersionRequirement::Valid
            );
        }
    }

    #[test]
    fn test_missing_segments() {
        // "2.2.7" → [2,2,7,0] < [2,2,7,6]
        let v = "2.2.7".to_string();
        assert_eq!(
            ArculusMinFirmwareVersionRequirement::new(v.clone()),
            ArculusMinFirmwareVersionRequirement::Invalid(v)
        );
    }

    #[test]
    fn test_extra_segments_ignored() {
        // only first four segments used: "2.2.7.6.1" → [2,2,7,6]
        let v = "2.2.7.6.1".to_string();
        assert_eq!(
            ArculusMinFirmwareVersionRequirement::new(v),
            ArculusMinFirmwareVersionRequirement::Valid
        );
    }

    #[test]
    fn test_non_numeric_segments() {
        // non-numeric → parse as zero: [2,0,0,0] < [2,2,7,6]
        let v = "2.x.y".to_string();
        assert_eq!(
            ArculusMinFirmwareVersionRequirement::new(v.clone()),
            ArculusMinFirmwareVersionRequirement::Invalid(v)
        );
    }

    #[test]
    fn test_sample_values() {
        assert_eq!(
            ArculusMinFirmwareVersionRequirement::sample(),
            ArculusMinFirmwareVersionRequirement::Valid
        );
        assert_eq!(
            ArculusMinFirmwareVersionRequirement::sample_other(),
            ArculusMinFirmwareVersionRequirement::Invalid(
                "2.2.7.4".to_string()
            )
        );
    }
}
