use crate::prelude::*;
use regex::Regex;

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct RnsDomain(pub String);

impl RnsDomain {
    pub fn new(raw_domain: String) -> Self {
        Self(raw_domain)
    }
}

impl HasSampleValues for RnsDomain {
    fn sample() -> Self {
        RnsDomain::new("sample1.xrd".to_owned())
    }

    fn sample_other() -> Self {
        RnsDomain::new("sample2.xrd".to_owned())
    }
}

impl RnsDomain {
    pub fn root_domain(&self) -> Result<RnsDomain> {
        let parts: Vec<&str> = self.0.split('.').collect();
        if parts.len() < 2 {
            return Err(CommonError::RnsInvalidDomain);
        }
        let root = parts[parts.len() - 2..].join(".");
        Ok(RnsDomain::new(root))
    }

    pub fn to_non_fungible_id(&self) -> Result<NonFungibleLocalId> {
        domain_to_non_fungible_id(&self.0)
    }

    pub fn validated(&self) -> Result<RnsDomain> {
        let raw = self.0.clone();

        let parts: Vec<&str> = raw.split('.').collect();

        if parts.len() < 2 {
            return Err(CommonError::RnsInvalidDomain);
        }

        if parts.last().copied() != Some("xrd") {
            return Err(CommonError::RnsInvalidDomain);
        }

        let label_regex =
            Regex::new(r"^[a-zA-Z0-9](?:[a-zA-Z0-9-]{0,63}[a-zA-Z0-9])?$")
                .map_err(|_| CommonError::RnsInvalidDomain)?;

        for label in &parts[..parts.len() - 1] {
            if label.len() < 2 || label.len() > 65 {
                return Err(CommonError::RnsInvalidDomain);
            }
            if label.contains('_') {
                return Err(CommonError::RnsInvalidDomain);
            }
            if !label_regex.is_match(label) {
                return Err(CommonError::RnsInvalidDomain);
            }
        }

        Ok(self.clone())
    }
}

const GRADIENT_PALETTE: [&str; 160] = [
    "#FF1744", "#FF1744", "#FF3D00", "#FF3D00", "#FF9100", "#FF9100",
    "#FFC400", "#FFC400", "#FFEA00", "#FFEA00", "#CDDC39", "#CDDC39",
    "#8BC34A", "#8BC34A", "#4CAF50", "#4CAF50", "#26C6DA", "#26C6DA",
    "#00BCD4", "#00BCD4", "#E57373", "#FF8A65", "#FFB74D", "#FFD54F",
    "#FFEE58", "#CDDC39", "#8BC34A", "#4CAF50", "#26C6DA", "#00BCD4",
    "#BA68C8", "#9575CD", "#7986CB", "#64B5F6", "#4FC3F7", "#4DD0E1",
    "#4DB6AC", "#81C784", "#AED581", "#DCE775", "#F06292", "#E91E63",
    "#9C27B0", "#673AB7", "#3F51B5", "#2196F3", "#03A9F4", "#00BCD4",
    "#009688", "#4CAF50", "#EF5350", "#FF7043", "#FFA726", "#FFC107",
    "#FFEB3B", "#CDDC39", "#8BC34A", "#4CAF50", "#26A69A", "#009688",
    "#FF1744", "#FF3D00", "#FF9100", "#FFC400", "#FFEA00", "#CDDC39",
    "#8BC34A", "#4CAF50", "#26C6DA", "#00BCD4", "#E53935", "#D32F2F",
    "#C2185B", "#7B1FA2", "#512DA8", "#303F9F", "#1976D2", "#0288D1",
    "#0097A7", "#00ACC1", "#4CAF50", "#8BC34A", "#CDDC39", "#FFEB3B",
    "#FFC107", "#FF9800", "#FF5722", "#795548", "#607D8B", "#333333",
    "#E57373", "#FF8A65", "#FFB74D", "#FFD54F", "#FFEE58", "#CDDC39",
    "#8BC34A", "#4CAF50", "#26C6DA", "#00BCD4", "#BA68C8", "#9575CD",
    "#7986CB", "#64B5F6", "#4FC3F7", "#4DD0E1", "#4DB6AC", "#81C784",
    "#AED581", "#DCE775", "#F06292", "#E91E63", "#9C27B0", "#673AB7",
    "#3F51B5", "#2196F3", "#03A9F4", "#00BCD4", "#009688", "#4CAF50",
    "#EF5350", "#FF7043", "#FFA726", "#FFC107", "#FFEB3B", "#CDDC39",
    "#8BC34A", "#4CAF50", "#26A69A", "#009688", "#FF1744", "#FF3D00",
    "#FF9100", "#FFC400", "#FFEA00", "#CDDC39", "#8BC34A", "#4CAF50",
    "#26C6DA", "#00BCD4", "#E53935", "#D32F2F", "#C2185B", "#7B1FA2",
    "#512DA8", "#303F9F", "#1976D2", "#0288D1", "#0097A7", "#00ACC1",
    "#4CAF50", "#8BC34A", "#CDDC39", "#FFEB3B", "#FFC107", "#FF9800",
    "#FF5722", "#795548", "#607D8B", "#333333",
];

impl RnsDomain {
    pub fn gradient_colors(&self) -> (String, String) {
        let input = &self.0;
        let char_sum: usize = input.chars().map(|c| c as usize).sum();
        let index1 = (char_sum + input.len()) % GRADIENT_PALETTE.len();
        let index2 = (char_sum * 2 + input.len()) % GRADIENT_PALETTE.len();

        let color1 = GRADIENT_PALETTE[index1].to_string();
        let color2 = GRADIENT_PALETTE[index2].to_string();
        (color1, color2)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_root_domain_subdomain() {
        // Valid domain with subdomain
        let domain = RnsDomain::new("sub.example.xrd".to_string());
        let root = domain.root_domain().unwrap();
        assert_eq!(root.0, "example.xrd");
    }

    #[test]
    fn test_root_domain_no_subdomain() {
        // Domain without subdomain
        let domain = RnsDomain::new("example.xrd".to_string());
        let root = domain.root_domain().unwrap();
        assert_eq!(root.0, "example.xrd");
    }

    #[test]
    fn test_valid_domain() {
        // Valid domain with one label and TLD "xrd"
        let domain = RnsDomain::new("example.xrd".to_string());
        assert!(domain.validated().is_ok());
    }

    #[test]
    fn test_valid_subdomain() {
        // Valid domain with subdomain
        let domain = RnsDomain::new("sub.example.xrd".to_string());
        assert!(domain.validated().is_ok());
    }

    #[test]
    fn test_invalid_domain_too_few_parts() {
        // A domain without a dot should be invalid.
        let domain = RnsDomain::new("example".to_string());
        assert_eq!(domain.validated(), Err(CommonError::RnsInvalidDomain));
    }

    #[test]
    fn test_invalid_domain_wrong_tld() {
        // Domain with TLD other than "xrd" should be rejected.
        let domain = RnsDomain::new("example.com".to_string());
        assert_eq!(domain.validated(), Err(CommonError::RnsInvalidDomain));
    }

    #[test]
    fn test_invalid_label_too_short() {
        // The first label must be at least 2 characters long.
        let domain = RnsDomain::new("e.xrd".to_string());
        assert_eq!(domain.validated(), Err(CommonError::RnsInvalidDomain));
    }

    #[test]
    fn test_invalid_label_too_long() {
        // Create a label with 66 characters (should be too long)
        let long_label = "a".repeat(66);
        let domain_str = format!("{}.xrd", long_label);
        let domain = RnsDomain::new(domain_str);
        assert_eq!(domain.validated(), Err(CommonError::RnsInvalidDomain));
    }

    #[test]
    fn test_invalid_label_with_underscore() {
        // Labels with underscores are not allowed.
        let domain = RnsDomain::new("exa_mple.xrd".to_string());
        assert_eq!(domain.validated(), Err(CommonError::RnsInvalidDomain));
    }

    #[test]
    fn test_invalid_label_bad_characters() {
        // Domain containing an invalid symbol (e.g. '@') should be rejected.
        let domain = RnsDomain::new("ex@mple.xrd".to_string());
        assert_eq!(domain.validated(), Err(CommonError::RnsInvalidDomain));
    }

    #[test]
    fn test_invalid_subdomain_missing_separator() {
        // A domain with an extra period leading to an empty label should fail.
        let domain = RnsDomain::new("example..xrd".to_string());
        // The empty label (between the two dots) fails the length check.
        assert_eq!(domain.validated(), Err(CommonError::RnsInvalidDomain));
    }

    #[test]
    fn test_gradient_colors_for_example_domain() {
        let domain_str = "example.xrd".to_string();
        let domain = RnsDomain::new(domain_str.clone());
        // Calling gradient_colors does not require validation.
        let (color1, color2) = domain.gradient_colors();
        // For "example.xrd", we expect colors based on our GRADIENT_PALETTE computation.
        // Calculation:
        // char_sum = sum("example.xrd" chars) = 1128, input.len() = 11.
        // index1 = (1128 + 11) % 160 = 1139 % 160 = 19, color = GRADIENT_PALETTE[19] => "#00BCD4"
        // index2 = ((1128 * 2) + 11) % 160 = 2267 % 160 = 27, color = GRADIENT_PALETTE[27] => "#4CAF50"
        assert_eq!(color1, "#00BCD4".to_string());
        assert_eq!(color2, "#4CAF50".to_string());
    }
}
