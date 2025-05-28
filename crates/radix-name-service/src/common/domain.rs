use crate::prelude::*;

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct Domain(String);

impl Domain {
    pub fn new(raw_domain: String) -> Self {
        Self(raw_domain)
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
        // Check that there are exactly two parts and the extension is "xrd"
        if parts.len() != 2 || parts[1] != "xrd" {
            return Err(CommonError::Unknown);
        }

        let name_part = parts[0];

        // Check length of the domain name (the part before the dot)
        if name_part.len() < 2 || name_part.len() > 65 {
            return Err(CommonError::Unknown);
        }

        // Check that no underscores are included
        if raw.contains('_') {
            return Err(CommonError::Unknown);
        }

        // Check domain format using regex
        let domain_format_regex = Regex::new(
        r"^(([a-zA-Z0-9]{2,})|([a-zA-Z]{2})|([a-zA-Z][0-9])|([0-9][a-zA-Z])|([a-zA-Z0-9][a-zA-Z0-9-_]{1,61}[a-zA-Z0-9]))\.xrd$"
    ).map_err(|_| CommonError::Unknown)?;

        if !domain_format_regex.is_match(&raw) {
            return Err(CommonError::Unknown);
        }

        Ok(self.clone())
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
}
