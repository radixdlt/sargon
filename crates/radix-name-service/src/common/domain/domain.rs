use crate::prelude::*;

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct Domain(pub String);

impl Domain {
    pub fn new(raw_domain: String) -> Self {
        Self(raw_domain)
    }
}

impl HasSampleValues for Domain {
    fn sample() -> Self {
        Domain::new("sample1.xrd".to_owned())
    }

    fn sample_other() -> Self {
        Domain::new("sample2.xrd".to_owned())
    }
}

use regex::Regex;

impl Domain {
    pub fn to_non_fungible_id(&self) -> Result<NonFungibleLocalId> {
        domain_to_non_fungible_id(&self.0, true)
    }

    pub fn validated(&self) -> Result<Domain> {
        let raw = self.0.clone();
        // Split the domain into parts by '.'
        let parts: Vec<&str> = raw.split('.').collect();
        // A valid domain must have at least one label and a TLD.
        if parts.len() < 2 {
            return Err(CommonError::Unknown);
        }
        // The last part must be "xrd"
        if parts.last().map(|s| *s) != Some("xrd") {
            return Err(CommonError::Unknown);
        }
        // Regex for a label: must start and end with alphanumeric characters.
        // Allows dashes or underscores in the middle (though we'll disallow underscores explicitly).
        // Length between 2 and 65 as in the original validation.
        let label_regex = Regex::new(r"^[a-zA-Z0-9](?:[a-zA-Z0-9-]{0,63}[a-zA-Z0-9])?$")
            .map_err(|_| CommonError::Unknown)?;
    
        // Check each label (excluding TLD)
        for label in &parts[..parts.len() - 1] {
            if label.len() < 2 || label.len() > 65 {
                return Err(CommonError::Unknown);
            }
            if label.contains('_') {
                return Err(CommonError::Unknown);
            }
            if !label_regex.is_match(label) {
                return Err(CommonError::Unknown);
            }
        }
    
        Ok(self.clone())
    }
}

const GRADIENT_PALETTE: [&str; 160] = [
    "#FF1744", "#FF1744", "#FF3D00", "#FF3D00", "#FF9100", "#FF9100", "#FFC400", "#FFC400", "#FFEA00", "#FFEA00",
        "#CDDC39", "#CDDC39", "#8BC34A", "#8BC34A", "#4CAF50", "#4CAF50", "#26C6DA", "#26C6DA", "#00BCD4", "#00BCD4",
        "#E57373", "#FF8A65", "#FFB74D", "#FFD54F", "#FFEE58", "#CDDC39", "#8BC34A", "#4CAF50", "#26C6DA", "#00BCD4",
        "#BA68C8", "#9575CD", "#7986CB", "#64B5F6", "#4FC3F7", "#4DD0E1", "#4DB6AC", "#81C784", "#AED581", "#DCE775",
        "#F06292", "#E91E63", "#9C27B0", "#673AB7", "#3F51B5", "#2196F3", "#03A9F4", "#00BCD4", "#009688", "#4CAF50",
        "#EF5350", "#FF7043", "#FFA726", "#FFC107", "#FFEB3B", "#CDDC39", "#8BC34A", "#4CAF50", "#26A69A", "#009688",
        "#FF1744", "#FF3D00", "#FF9100", "#FFC400", "#FFEA00", "#CDDC39", "#8BC34A", "#4CAF50", "#26C6DA", "#00BCD4",
        "#E53935", "#D32F2F", "#C2185B", "#7B1FA2", "#512DA8", "#303F9F", "#1976D2", "#0288D1", "#0097A7", "#00ACC1",
        "#4CAF50", "#8BC34A", "#CDDC39", "#FFEB3B", "#FFC107", "#FF9800", "#FF5722", "#795548", "#607D8B", "#333333",
        "#E57373", "#FF8A65", "#FFB74D", "#FFD54F", "#FFEE58", "#CDDC39", "#8BC34A", "#4CAF50", "#26C6DA", "#00BCD4",
        "#BA68C8", "#9575CD", "#7986CB", "#64B5F6", "#4FC3F7", "#4DD0E1", "#4DB6AC", "#81C784", "#AED581", "#DCE775",
        "#F06292", "#E91E63", "#9C27B0", "#673AB7", "#3F51B5", "#2196F3", "#03A9F4", "#00BCD4", "#009688", "#4CAF50",
        "#EF5350", "#FF7043", "#FFA726", "#FFC107", "#FFEB3B", "#CDDC39", "#8BC34A", "#4CAF50", "#26A69A", "#009688",
        "#FF1744", "#FF3D00", "#FF9100", "#FFC400", "#FFEA00", "#CDDC39", "#8BC34A", "#4CAF50", "#26C6DA", "#00BCD4",
        "#E53935", "#D32F2F", "#C2185B", "#7B1FA2", "#512DA8", "#303F9F", "#1976D2", "#0288D1", "#0097A7", "#00ACC1",
        "#4CAF50", "#8BC34A", "#CDDC39", "#FFEB3B", "#FFC107", "#FF9800", "#FF5722", "#795548", "#607D8B", "#333333"
];

impl Domain {
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
mod test {
    use super::*;

    #[test]
    fn test_domain_validated() {
        let domain = Domain::new("example.xrd".to_string());
        assert!(domain.validated().is_ok());

        let invalid_domain = Domain::new("e.xrd".to_string());
        assert!(invalid_domain.validated().is_err());

        let invalid_domain2 = Domain::new("example_xrd".to_string());
        assert!(invalid_domain2.validated().is_err());

        let invalid_domain3 = Domain::new("example.xrd_invalid".to_string());
        assert!(invalid_domain3.validated().is_err());
    }

    #[test]
    fn test_subdomain_validated() {
        let subdomain = Domain::new("sub.example.xrd".to_string());
        assert!(subdomain.validated().is_ok());

        let invalid_subdomain = Domain::new("sub.example_xrd".to_string());
        assert!(invalid_subdomain.validated().is_err());

        let invalid_subdomain2 = Domain::new("sub.example.xrd_invalid".to_string());
        assert!(invalid_subdomain2.validated().is_err());
    }
}
